use crate::contracts::{erc20::*, eth::*};
use alloy::primitives::Address;
use anyhow::Result;
use rig_derive::rig_tool;
use std::str::FromStr;

#[rig_tool(
    description = "Query ETH balance for an account",
    params(
        chain = "The chain name, such as arbitrum",
        account_address = "The address of the account to query balance for"
    )
)]
pub async fn get_eth_balance(
    chain: String,
    account_address: String,
) -> Result<String, rig::tool::ToolError> {
    let account_address = Address::from_str(&account_address)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid account_address format".into()))?;

    println!(
        "chain_name: {}, account_address: {}",
        chain, account_address
    );

    let result = eth_balance(account_address, chain).await;
    match result {
        Ok(balance) => Ok(balance.to_string()),
        Err(e) => Err(rig::tool::ToolError::ToolCallError(
            format!("get_eth_balance error: {}", e).into(),
        )),
    }
}

#[rig_tool(
    description = "Query ERC20 token balance for an account",
    params(
        chain = "The chain name, such as arbitrum",
        token_address = "The address of the ERC20 token contract",
        account_address = "The address of the account to query balance for"
    )
)]
pub async fn get_balance(
    chain: String,
    token_address: String,
    account_address: String,
) -> Result<String, rig::tool::ToolError> {
    let token_address = Address::from_str(&token_address)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid token_address format".into()))?;

    let account_address = Address::from_str(&account_address)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid account_address format".into()))?;

    println!(
        "chain_name: {}, token_address: {}, account_address: {}",
        chain, token_address, account_address
    );

    let result = balance_of_erc20(account_address, token_address, chain).await;
    match result {
        Ok(balance) => Ok(balance.to_string()),
        Err(e) => Err(rig::tool::ToolError::ToolCallError(
            format!("balance_of_erc20 error: {}", e).into(),
        )),
    }
}
