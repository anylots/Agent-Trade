use crate::wallets::send_tx;
use alloy::primitives as alloy_primitives;
use alloy::providers::Provider;
use alloy::{
    primitives::{Address, TxHash, B256},
    rpc::types::TransactionRequest,
};
use anyhow::{anyhow, Result};

use uniswap_sdk_core::{prelude::*, token};
use uniswap_v3_sdk::prelude::{sdk_core::prelude::CurrencyAmount, *};

use super::common::make_provider;

pub async fn swap(
    input_token: Address,
    output_token: Address,
    input_amount: u128,
    chain_name: String,
) -> std::result::Result<B256, anyhow::Error> {
    let provider = make_provider(chain_name).map_err(|e| {
        rig::tool::ToolError::ToolCallError(format!("Provider error: {}", e).into())
    })?;

    // Sync send transfer call.
    let tx_hash: std::result::Result<TxHash, anyhow::Error> = async move {
        let handle = tokio::task::spawn_blocking(move || {
            tokio::runtime::Handle::current().block_on(async {
                let tx =
                    create_swap_request(input_token, output_token, input_amount, &provider).await?;
                // Send the transaction and return the transaction hash
                send_tx(tx, provider).await
            })
        });
        match handle.await {
            Ok(Ok(tx)) => Ok(tx.transaction_hash),
            Ok(Err(e)) => Err(anyhow!(format!("alloy rpc error: {}", e))),
            Err(e) => Err(anyhow!(format!("tokio exec error: {}", e))),
        }
    }
    .await;
    tx_hash
}

async fn create_swap_request(
    input_token: Address,
    output_token: Address,
    input_amount: u128,
    provider: &alloy::providers::RootProvider,
) -> Result<TransactionRequest, anyhow::Error> {
    let chain_id = provider.get_chain_id().await?;

    let pool = Pool::<EphemeralTickMapDataProvider>::from_pool_key_with_tick_data_provider(
        chain_id,
        FACTORY_ADDRESS,
        input_token,
        output_token,
        FeeAmount::MEDIUM,
        provider.clone(),
        None,
    )
    .await
    .map_err(|e| anyhow!(format!("Pool creation error: {}", e)))?;
    let amount_in = CurrencyAmount::from_raw_amount(
        token!(chain_id, input_token, 18, "INPUT_TOKEN", "INPUT_TOKEN"),
        input_amount,
    )?;

    let route = Route::new(
        vec![pool],
        token!(chain_id, input_token, 18, "INPUT_TOKEN", "INPUT_TOKEN"),
        token!(chain_id, output_token, 18, "OUTPUT_TOKEN", "OUTPUT_TOKEN"),
    );

    let params = quote_call_parameters(&route, &amount_in, TradeType::ExactInput, None);
    let tx = TransactionRequest::default()
        .to(*QUOTER_ADDRESSES
            .get(&chain_id)
            .expect("chain supported by uniswap"))
        .input(params.calldata.into());
    Ok(tx)
}

#[test]
fn test_swap() {}
