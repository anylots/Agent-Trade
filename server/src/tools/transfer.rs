use crate::contracts::{erc20::*, eth::transfer_eth};
use alloy::primitives::Address;
use anyhow::Result;
use rig_derive::rig_tool;
use std::str::FromStr;

const MAX_AMOUNT: u128 = 10u128.pow(5);

const MAX_ETH_AMOUNT: u128 = 10u128; //maximum amount in ETH

#[rig_tool(
    description = "Transfer ETH to a specific address",
    params(
        chain = "The chain name, such as arbitrum",
        to_address = "The receiving address",
        amount = "The amount of ETH to transfer"
    )
)]
pub async fn eth_transfer(
    chain: String,
    to_address: String,
    amount: String,
) -> Result<String, rig::tool::ToolError> {
    let to_address = Address::from_str(&to_address)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid to_address format".into()))?;

    let amount = u128::from_str(&amount)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid amount format".into()))?;

    println!(
        "chain_name: {}, to_address: {}, amount: {}",
        chain, to_address, amount
    );

    if amount > MAX_ETH_AMOUNT {
        println!(
            "amount = {} exceeds the safe value = {}",
            amount, MAX_ETH_AMOUNT
        );
        return Err(rig::tool::ToolError::ToolCallError(
            format!(
                "amount = {} exceeds the safe value = {}",
                amount, MAX_ETH_AMOUNT
            )
            .into(),
        ));
    }

    let result = transfer_eth(to_address, amount, chain).await;
    match result {
        Ok(h) => Ok(h.to_string()),
        Err(e) => Err(rig::tool::ToolError::ToolCallError(
            format!("transfer_eth error: {}", e).into(),
        )),
    }
}

#[rig_tool(
    description = "Transfer ERC20 tokens to a specific address",
    params(
        chain = "The chain name, such as arbitrum",
        token_address = "The address of the ERC20 token contract",
        to_address = "The receiving address",
        amount = "The amount of tokens to transfer"
    )
)]
pub async fn erc20_transfer(
    chain: String,
    token_address: String,
    to_address: String,
    amount: String,
) -> Result<String, rig::tool::ToolError> {
    let token_address = Address::from_str(&token_address)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid token_address format".into()))?;

    let to_address = Address::from_str(&to_address)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid to_address format".into()))?;

    let amount = u128::from_str(&amount)
        .map_err(|_| rig::tool::ToolError::ToolCallError("Invalid amount format".into()))?;

    println!(
        "chain_name: {}, token_address: {}, to_address: {}, amount: {}",
        chain, token_address, to_address, amount
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

    let result = transfer_erc20(to_address, amount, token_address, chain).await;
    match result {
        Ok(h) => Ok(h.to_string()),
        Err(e) => Err(rig::tool::ToolError::ToolCallError(
            format!("transfer_erc20 error: {}", e).into(),
        )),
    }
}


#[cfg(test)]
mod test {
    use crate::chains::CHAIN_INFOS;
    use crate::tools::transfer::{Erc20Transfer, EthTransfer};
    use anyhow::Result;
    use rig::completion::Prompt;
    use rig::providers::openai;

    #[tokio::test]
    async fn test_run() -> Result<()> {
        // Create OpenAI client and model
        let openai_client = openai::Client::from_url("sk-xxxxx", "https://api.xxxxx.xx/");

        //Qwen/Qwen2.5-32B-Instruct
        //Qwen/Qwen2.5-72B-Instruct-128K
        let transfer_agent = openai_client
            .agent("Qwen/Qwen2.5-32B-Instruct")
            .preamble(
                "You are a transfer agent here to help the user perform ERC20 token transfers.",
            )
            .context(&serde_json::to_string(&*CHAIN_INFOS).unwrap())
            .max_tokens(2048)
            .tool(Erc20Transfer)
            .build();

        // Prompt the agent and print the response
        println!("Transfer ERC20 tokens");
        println!(
            "Transfer Agent: {}",
            transfer_agent
                .prompt("Transfer 10 USDC to 0x1CBd0109c7452926fC7cCf06e73aCC505A296cc7 on base")
                .await?
        );
        Ok(())
    }

    #[tokio::test]
    async fn test_run_eth() -> Result<()> {
        // Create OpenAI client and model
        let openai_client = openai::Client::from_url("sk-xxxxx", "https://api.xxxxx.xx/");

        let transfer_agent = openai_client
            .agent("Qwen/Qwen2.5-32B-Instruct")
            .preamble("You are a transfer agent here to help the user perform ETH transfers.")
            .context(&serde_json::to_string(&*CHAIN_INFOS).unwrap())
            .max_tokens(2048)
            .tool(EthTransfer)
            .build();

        // Prompt the agent and print the response
        println!("Transfer ETH");
        println!(
            "Transfer Agent: {}",
            transfer_agent
                .prompt("Transfer 10 ETH to 0x1CBd0109c7452926fC7cCf06e73aCC505A296cc7 on base")
                .await?
        );
        Ok(())
    }
}
