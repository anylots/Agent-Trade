use super::common::make_provider;
use alloy::{
    network::{EthereumWallet, TransactionBuilder},
    primitives::{utils::parse_ether, Address, TxHash, B256, U256},
    providers::{Provider, ProviderBuilder},
    rpc::types::TransactionRequest,
    signers::local::PrivateKeySigner,
};
use anyhow::anyhow;
use std::sync::Arc;

pub async fn eth_balance(
    account_address: Address,
    chain_name: String,
) -> std::result::Result<U256, anyhow::Error> {
    let balance: std::result::Result<U256, anyhow::Error> = async move {
        let handle = tokio::task::spawn_blocking(move || {
            let result = tokio::runtime::Handle::current().block_on(async {
                let provider = make_provider(chain_name)?;
                provider.get_balance(account_address).await.map_err(|e| {
                    anyhow!(format!("get_eth_balance error: {}", e))
                })
            });
            result
        });

        match handle.await {
            Ok(result) => result,
            Err(e) => Err(anyhow!(format!("tokio exec error: {}", e))),
        }
    }
    .await;

    balance
}

pub async fn transfer_eth(
    to_address: Address,
    amount: u128,
    chain_name: String,
) -> std::result::Result<B256, anyhow::Error> {
    // Read the private key from the environment variable
    // let private_key = env::var("PRIVATE_KEY").unwrap();

    // [RISK WARNING! Writing a private key in the code file is insecure behavior.]
    // The following code is for testing only. Set up signer from private key, be aware of danger.
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer: PrivateKeySigner = private_key.parse().expect("parse PrivateKeySigner");
    let wallet: EthereumWallet = EthereumWallet::from(signer.clone());

    // Create a http client to the EVM chain network.
    let provider = make_provider(chain_name)?;

    // Create eth signer.
    let signer = Arc::new(ProviderBuilder::new().wallet(wallet).on_provider(provider));

    // Sync send transfer call.
    let tx_hash: std::result::Result<TxHash, anyhow::Error> = async move {
        let handle = tokio::task::spawn_blocking(move || {
            tokio::runtime::Handle::current().block_on(async {
                let tx = TransactionRequest::default()
                    .with_to(to_address)
                    .with_value(parse_ether(&amount.to_string()).unwrap_or_default());

                // Send the transaction and listen for the transaction to be included.
                signer.send_transaction(tx).await
            })
        });
        match handle.await {
            Ok(Ok(tx)) => Ok(*tx.tx_hash()),
            Ok(Err(e)) => Err(anyhow!(format!("alloy rpc error: {}", e))),
            Err(e) => Err(anyhow!(format!("tokio exec error: {}", e))),
        }
    }
    .await;
    tx_hash
}

#[cfg(test)]
mod test {
    use crate::contracts::eth::transfer_eth;
    use alloy::primitives::Address;
    use anyhow::Result;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_transfer_eth() -> Result<()> {
        let to_address = Address::from_str("1CBd0109c7452926fC7cCf06e73aCC505A296cc7").unwrap();
        let tx_hash = transfer_eth(to_address, 10, String::from("http://localhost:8545")).await;
        println!("tx_hash:{}", tx_hash.unwrap());
        Ok(())
    }
}
