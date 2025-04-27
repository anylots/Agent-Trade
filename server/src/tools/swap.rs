use crate::contracts::{erc20::*, uniswap::swap};
use alloy::primitives::Address;
use anyhow::Result;
use rig_derive::rig_tool;
use std::str::FromStr;

const MAX_SWAP_AMOUNT: u128 = 10u128.pow(5);

#[rig_tool(
    description = "Approve an allowance for a spender",
    params(
        chain = "The chain name, such as arbitrum",
        token_address = "The address of the ERC20 token contract",
        spender_address = "The address of the spender",
        amount = "The amount of tokens to approve"
    )
)]
pub async fn approve(
    chain: String,
    token_address: String,
    spender_address: String,
    amount: String,
) -> Result<String, rig::tool::ToolError> {
    let token_address = Address::from_str(&token_address)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid token_address format".into()))?;

    let spender_address = Address::from_str(&spender_address).map_err(|_| {
        rig::tool::ToolError::ToolCallError("Invalid spender_address format".into())
    })?;

    let amount = u128::from_str(&amount)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid amount format".into()))?;

    println!(
        "chain_name: {}, token_address: {}, spender_address: {}, amount: {}",
        chain, token_address, spender_address, amount
    );

    let result = approve_erc20(spender_address, amount, token_address, chain).await;
    match result {
        Ok(h) => Ok(h.to_string()),
        Err(e) => Err(rig::tool::ToolError::ToolCallError(
            format!("approve_erc20 error: {}", e).into(),
        )),
    }
}

#[rig_tool(
    description = "Check the current allowance for a spender",
    params(
        chain = "The chain name, such as arbitrum",
        token_address = "The address of the ERC20 token contract",
        owner_address = "The address of the token owner",
        spender_address = "The address of the spender"
    )
)]
pub async fn check_allowance(
    chain: String,
    token_address: String,
    owner_address: String,
    spender_address: String,
) -> Result<String, rig::tool::ToolError> {
    let token_address = Address::from_str(&token_address)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid token_address format".into()))?;

    let owner_address = Address::from_str(&owner_address)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid owner_address format".into()))?;

    let spender_address = Address::from_str(&spender_address).map_err(|_| {
        rig::tool::ToolError::ToolCallError("Invalid spender_address format".into())
    })?;

    println!(
        "chain_name: {}, token_address: {}, owner_address: {}, spender_address: {}",
        chain, token_address, owner_address, spender_address
    );

    let result = check_allowance_erc20(owner_address, spender_address, token_address, chain).await;
    match result {
        Ok(allowance) => Ok(allowance.to_string()),
        Err(e) => Err(rig::tool::ToolError::ToolCallError(
            format!("check_allowance_erc20 error: {}", e).into(),
        )),
    }
}

#[rig_tool(
    description = "Swap ERC20 tokens on Uniswap",
    params(
        input_token = "Contract address of the input token",
        output_token = "Contract address of the output token",
        input_amount = "Amount of input token",
        chain = "Chain name, e.g. arbitrum"
    )
)]
pub async fn uniswap_v3_swap(
    input_token: String,
    output_token: String,
    input_amount: String,
    chain: String,
) -> Result<String, rig::tool::ToolError> {
    let input_token = Address::from_str(&input_token).map_err(|_| {
        rig::tool::ToolError::ToolCallError("Invalid input_token address format".into())
    })?;

    let output_token = Address::from_str(&output_token).map_err(|_| {
        rig::tool::ToolError::ToolCallError("Invalid output_token address format".into())
    })?;

    let input_amount = u128::from_str(&input_amount)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid input_amount format".into()))?;

    if input_amount > MAX_SWAP_AMOUNT {
        return Err(rig::tool::ToolError::ToolCallError(
            format!(
                "input_amount={} exceeds the maximum safe value={}",
                input_amount, MAX_SWAP_AMOUNT
            )
            .into(),
        ));
    }

    let swap_result = swap(input_token, output_token, input_amount, chain).await;
    match swap_result {
        Ok(tx_hash) => Ok(tx_hash.to_string()),
        Err(e) => Err(rig::tool::ToolError::ToolCallError(
            format!("swap error: {}", e).into(),
        )),
    }
}
