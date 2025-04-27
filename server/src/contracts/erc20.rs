use crate::wallets::send_tx;
use alloy::{
    primitives::{Address, TxHash, B256, U256},
    rpc::types::TransactionRequest,
};
use anyhow::anyhow;

use super::{abi::IERC20, common::make_provider};

pub async fn approve_erc20(
    spender_address: Address,
    amount: u128,
    token_address: Address,
    chain_name: String,
) -> std::result::Result<B256, anyhow::Error> {
    let tx_hash: std::result::Result<TxHash, anyhow::Error> = async move {
        let handle = tokio::task::spawn_blocking(move || {
            let result = tokio::runtime::Handle::current().block_on(async {
                let provider = make_provider(chain_name)?;
                let erc20 = IERC20::IERC20Instance::new(token_address, provider.clone());
                let request: TransactionRequest = erc20
                    .approve(spender_address, U256::from(amount))
                    .into_transaction_request();
                send_tx(request, provider).await
            });
            result
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

pub async fn check_allowance_erc20(
    owner_address: Address,
    spender_address: Address,
    token_address: Address,
    chain_name: String,
) -> std::result::Result<U256, anyhow::Error> {
    let allowance: std::result::Result<U256, anyhow::Error> = async move {
        let handle = tokio::task::spawn_blocking(move || {
            let result = tokio::runtime::Handle::current().block_on(async {
                let provider = make_provider(chain_name)?;
                let erc20 = IERC20::IERC20Instance::new(token_address, provider.clone());
                match erc20.allowance(owner_address, spender_address).call().await {
                    Ok(allowance) => Ok(allowance._0),
                    Err(e) => Err(anyhow!(format!("allowance call error: {}", e))),
                }
            });
            result
        });

        match handle.await {
            Ok(result) => result,
            Err(e) => Err(anyhow!(format!("tokio exec error: {}", e))),
        }
    }
    .await;

    allowance
}

pub async fn balance_of_erc20(
    account_address: Address,
    token_address: Address,
    chain_name: String,
) -> std::result::Result<U256, anyhow::Error> {
    let balance: std::result::Result<U256, anyhow::Error> = async move {
        let handle = tokio::task::spawn_blocking(move || {
            let result = tokio::runtime::Handle::current().block_on(async {
                let provider = make_provider(chain_name)?;
                let erc20 = IERC20::IERC20Instance::new(token_address, provider.clone());
                match erc20.balanceOf(account_address).call().await {
                    Ok(balance) => Ok(balance._0),
                    Err(e) => Err(anyhow!(format!("balanceOf call error: {}", e))),
                }
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

pub async fn transfer_erc20(
    to_address: Address,
    amount: u128,
    token_address: Address,
    chain_name: String,
) -> std::result::Result<B256, anyhow::Error> {
    // Sync send transfer call.
    let tx_hash: std::result::Result<TxHash, anyhow::Error> = async move {
        let handle = tokio::task::spawn_blocking(move || {
            let result = tokio::runtime::Handle::current().block_on(async {
                // Create a http client to the EVM chain network.
                let provider = make_provider(chain_name)?;
                // Create contract instance.
                let erc20 = IERC20::IERC20Instance::new(token_address, provider.clone());
                let decimal = erc20.decimals().call().await.unwrap()._0;
                let request: TransactionRequest = erc20
                    .transfer(to_address, U256::from(amount * 10u128.pow(decimal.into())))
                    .into_transaction_request();
                send_tx(request, provider).await
            });
            result
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

#[cfg(test)]
mod test {
    use crate::contracts::erc20::transfer_erc20;
    use alloy::primitives::Address;
    use anyhow::Result;
    use std::str::FromStr;

    #[tokio::test]
    async fn test_transfer_erc20() -> Result<()> {
        let to_address = Address::from_str("1CBd0109c7452926fC7cCf06e73aCC505A296cc7").unwrap();
        let token_address = Address::from_str("5FbDB2315678afecb367f032d93F642f64180aa3").unwrap();
        let tx_hash = transfer_erc20(
            to_address,
            10,
            token_address,
            String::from("http://localhost:8545"),
        )
        .await;
        println!("tx_hash:{}", tx_hash.unwrap());
        Ok(())
    }
}
