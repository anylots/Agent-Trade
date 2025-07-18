use anyhow::Result;
use agent_trade::chains::CHAIN_INFOS;
use agent_trade::tools::transfer::Erc20Transfer;
use rig::completion::Prompt;
use rig::providers::openai;

#[tokio::main]
async fn main() -> Result<()> {
    // Create OpenAI client and model
    let openai_client = openai::Client::from_url("sk-xxxxx", "https://api.xxxxx.xx/");

    // agent
    let transfer_agent = openai_client
        .agent("Qwen/Qwen2.5-32B-Instruct")
        .preamble("You are a transfer agent here to help the user perform ERC20 token transfers.")
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
