use anyhow::Result;
use rig::completion::Prompt;
// use rig::providers::openai;
use rig::providers::openrouter::Client;

use crate::utils::OPENROUTER_API_KEY;

pub async fn prompt(prompt: &str, model: &str) -> Result<String> {
    // Create OpenAI client and model
    let openrouter = Client::new(OPENROUTER_API_KEY.as_str());

    let agent = openrouter.agent(model).max_tokens(10240).build();
    Ok(agent.prompt(prompt).await?)
}

#[tokio::test]
async fn test_prompt() {
    dotenv::dotenv().ok();

    let msg = "Extract the Token symbol from the following information and put it into a array (You only give the final data results):
**Trending Crypto Tokens on Twitter (Last 24h)**\n\n**Method:** `get_trending_tokens`\n**Time Window:** `24h`\n**Total Tokens Returned:**
 `50`\n**Page:** `1`\n**Page Size:** `50`\n\n**Top 16 Popular Tokens (by Current Count):**\n\n1. **TROLL**: \n\t* Current Count: `463`\n\t* Previous Count: `477`\n\t* Change (%): **-2.94%**\n2. **BTC**: \n\t* Current Count: `458`\n\t* Previous Count: `500`\n\t* Change (%): **-8.4%**\n3. **ETH**: \n\t* Current Count: `323`\n\t* Previous Count: `247`\n\t* Change (%): **+30.77%**\n4. **DOG**: \n\t* Current Count: `222`\n\t* Previous Count: `158`\n\t* Change (%): **+40.51%**\n5. **SOL**: \n\t* Current Count: `176`\n\t* Previous Count: `207`\n\t* Change (%): **-14.98%**\n6. **PENGU**: \n\t* Current Count: `123`\n\t* Previous Count: `189`\n\t* Change (%): **-34.92%**\n7. **HOUSE**: \n\t* Current Count: `118`\n\t* Previous Count: `79`\n\t* Change (%): **+49.37%**\n8. **LEMON**: \n\t* Current Count: `117`\n\t* Previous Count: `136`\n\t* Change (%): **-13.97%**\n9. **AMZN**: \n\t* Current Count: `100`\n\t* Previous Count: `30`\n\t* Change (%): **+233.33%**\n10. **USDC**: \n\t* Current Count: `90`\n\t* Previous Count: `93`\n\t* Change (%): **-3.23%**\n11. **XMW**: \n\t* Current Count: `82`\n\t* Previous Count: `78`\n\t* Change (%): **+5.13%**\n12. **MVG**: \n\t* Current Count: `77`\n\t* Previous Count: `33`\n\t* Change (%): **+133.33%**\n13. **SUI**: \n\t* Current Count: `77`\n\t* Previous Count";
    let result = prompt(msg, "qwen/qwen-2.5-72b-instruct").await.unwrap();
    //test_prompt result: "```json\n[\"TROLL\", \"BTC\", \"ETH\", \"DOG\", \"SOL\", \"PENGU\", \"HOUSE\", \"LEMON\", \"AMZN\", \"USDC\", \"XMW\", \"MVG\", \"SUI\"]\n```"

    println!("test_prompt result: {:?}", result);

    // Extract token symbols from the JSON string
    let tokens: Vec<&str> = result
        .trim_start_matches("```json\n")
        .trim_end_matches("\n```")
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().trim_matches('"'))
        .collect();

    println!("Tokens length: {}", tokens.len());
    println!("Tokens: {:?}", tokens);
}
