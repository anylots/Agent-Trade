use crate::chains::get_chain_info;
use alloy::{
    network::EthereumWallet,
    primitives::{utils::parse_ether, Address, TxHash, B256, U256},
    providers::{ProviderBuilder, WalletProvider},
    signers::local::PrivateKeySigner,
    sol,
};
use anyhow::{anyhow, Result};
use rig_derive::rig_tool;
use serde::{Deserialize, Serialize};
use std::{str::FromStr, sync::Arc};

use super::common::make_provider;

const MAX_AMOUNT: u128 = 10u128;

sol! {
    #[derive(Debug, PartialEq, Eq, Deserialize, Serialize)]
    #[sol(rpc)]
    interface IROUTER {
        function swapExactETHForTokens(uint amountOutMin, address[] calldata path, address to, uint deadline) payable returns (uint[] memory amounts);
        function getAmountsOut(uint amountIn, address[] memory path)public view returns (uint[] memory amounts);
    }
}

#[rig_tool(
    description = "Swap ETH for a specific ERC20 token",
    params(
        chain = "The chain name, such as arbitrum",
        token_address = "The address of the ERC20 token to receive",
        amount = "The amount of ETH to swap"
    )
)]
pub async fn eth_swap_to_erc20(
    chain: String,
    token_address: String,
    amount: String,
) -> Result<String, rig::tool::ToolError> {
    let token_address = Address::from_str(&token_address)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid token_address format".into()))?;

    let amount = u128::from_str(&amount)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid amount format".into()))?;

    println!(
        "chain_name: {}, token_address: {}, amount: {}",
        chain, token_address, amount
    );

    if amount > MAX_AMOUNT {
        println!(
            "amount = {} exceeds the safe value = {}",
            amount, MAX_AMOUNT
        );
        return Err(rig::tool::ToolError::ToolCallError(
            format!(
                "amount = {} exceeds the safe value = {}",
                amount, MAX_AMOUNT
            )
            .into(),
        ));
    }

    let chain_info = get_chain_info(&chain).ok_or(rig::tool::ToolError::ToolCallError(
        "get_chain_info none".into(),
    ))?;

    let weth = chain_info
        .tokens
        .iter()
        .find(|t| t.0 == "WETH")
        .ok_or(rig::tool::ToolError::ToolCallError(
            "WETH token not found".into(),
        ))?
        .1;

    let path: Vec<Address> = vec![
        Address::from_str(weth).map_err(|_| {
            rig::tool::ToolError::ToolCallError("Invalid WETH address format".into())
        })?,
        token_address,
    ]; // ETH -> Token

    let result = swap_eth_to_erc20(
        Address::from_str(&chain_info.swap_router).map_err(|_| {
            rig::tool::ToolError::ToolCallError("Invalid swap_router address format".into())
        })?,
        parse_ether(&amount.to_string()).unwrap_or_default(),
        path,
        chain,
    )
    .await;

    match result {
        Ok(h) => Ok(h.to_string()),
        Err(e) => Err(rig::tool::ToolError::ToolCallError(
            format!("swap_eth_to_erc20 error: {}", e).into(),
        )),
    }
}

async fn swap_eth_to_erc20(
    router_address: Address,
    amount: U256,
    path: Vec<Address>,
    chain_name: String,
) -> std::result::Result<B256, anyhow::Error> {
    let private_key = "0xac0974bec39a17e36ba4a6b4d238ff944bacb478cbed5efcae784d7bf4f2ff80";
    let signer: PrivateKeySigner = private_key.parse().expect("parse PrivateKeySigner");
    let wallet: EthereumWallet = EthereumWallet::from(signer.clone());

    let provider = make_provider(chain_name)?;

    let eth_signer = Arc::new(
        ProviderBuilder::new()
            .wallet(wallet)
            .on_provider(provider.clone()),
    );

    // Create contract instance.
    let swap_router_instance = IROUTER::IROUTERInstance::new(router_address, eth_signer.clone());

    // Prepare swap func params.
    let receive_address = eth_signer.default_signer_address();
    let deadline = U256::from(
        std::time::SystemTime::now()
            .duration_since(std::time::SystemTime::UNIX_EPOCH)
            .expect("Time error")
            .as_secs()
            + 1200,
    ); // 20 minutes deadline

    let tx_hash: std::result::Result<TxHash, anyhow::Error> = async move {
        let handle = tokio::task::spawn_blocking(move || {
            
            tokio::runtime::Handle::current().block_on(async {
                let expected_amount: U256 = swap_router_instance
                    .getAmountsOut(amount, path.clone())
                    .call()
                    .await
                    .unwrap()
                    .amounts[1];
                //Calculate amount_out_min (for example, set a slippage of 0.5%)
                let slippage = U256::from(5); // 0.5%
                let amount_out_min =
                    expected_amount * (U256::from(1000) - slippage) / U256::from(1000);

                swap_router_instance
                    .swapExactETHForTokens(amount_out_min, path, receive_address, deadline)
                    .send()
                    .await
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

#[tokio::test]
async fn test_swap_eth_to_erc20() -> Result<()> {
    let weth = Address::from_str("0xC02aaA39b223FE8D0A0e5C4F27eAD9083C756Cc2").unwrap();
    let expect_token = Address::from_str("5FbDB2315678afecb367f032d93F642f64180aa3").unwrap();
    let path: Vec<Address> = vec![weth, expect_token]; // ETH -> Token

    let amount = "0.1".to_string(); // 0.1 ETH
    let tx_hash = swap_eth_to_erc20(
        Address::from_str("0x7a250d5630B4cF539739dF2C5dAcb4c659F2488D").unwrap(),
        parse_ether(&amount).unwrap(),
        path,
        String::from("http://localhost:8545"),
    )
    .await;
    println!("tx_hash:{}", tx_hash.unwrap());
    Ok(())
}

#[tokio::test]
async fn test_run() -> Result<()> {
    use crate::chains::CHAIN_INFOS;
    use rig::completion::Prompt;
    use rig::providers::openai;

    let openai_client = openai::Client::from_url("sk-xxxxx", "https://api.xxxxx.xx/");

    // Define the agent with the swap tool.
    let swap_agent = openai_client
        .agent("Qwen/Qwen2.5-32B-Instruct")
        .preamble("You are a swap agent here to help the user perform ETH to ERC20 token swaps.")
        .context(&serde_json::to_string(&*CHAIN_INFOS).unwrap())
        .max_tokens(2048)
        .tool(EthSwapToErc20)
        .build();

    // Prompt the agent and print the response.
    println!("Swap ETH to ERC20 token");
    println!(
        "Swap Agent: {}",
        swap_agent.prompt("Swap 0.1 ETH to USDC on base").await?
    );
    Ok(())
}
