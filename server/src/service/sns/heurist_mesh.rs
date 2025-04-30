use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::{
    service::openrouter::preamble::prompt,
    utils::{HEURIST_API_KEY, HEURIST_MESH_URL},
};

/// Response structure for Heurist API
#[derive(Deserialize, Debug)]
struct HeuristResponse {
    response: String,
    data: Value,
}

/// Input structure for Heurist API requests
#[derive(Serialize, Debug)]
struct HeuristInput {
    query: String,
}

/// Request body structure for Heurist API
#[derive(Serialize, Debug)]
struct HeuristRequestBody {
    agent_id: String,
    input: HeuristInput,
}

const EXCLUSION_LIST: &[&str] = &[
    "BTC", "ETH", "USDT", "XRP", "BNB", "SOL", "USDC", "DOGE", "ADA", "TRX", "STETH", "WBTC",
    "SUI", "LINK", "AVAX", "XLM", "LEO", "TON", "SHIB", "HBAR", "USDS", "WSTETH", "BCH", "LTC",
    "DOT", "HYPE", "FDUSD", "TRUMP", "WSOL", "ALPACA", "PUNDIX", "SIGN", "PEPE", "WETH", "VIRTUA",
    "INIT", "BONK", "LAYER", "USUAL", "WLD", "FARTCOIN", "USD1", "SUSDE", "POL", "ALGO", "ATOM",
    "LBTC", "FET", "FTN", "FIL", "ENA", "TIA", "S", "ARB", "JLP", "SOLVBTC", "KCS", "JUP", "MKR",
    "OP", "XDC", "STX", "NEXO", "BNSOL", "FLR", "QNT", "ZK", "STRK", "TAO", "DOG", "USD",
];

/// Call the Heurist Mesh API with the specified agent ID and query
///
/// # Arguments
/// * `agent_id` - The ID of the Heurist agent to call, eg: DexScreenerTokenInfoAgent
/// * `query` - The query string to send to the agent
///
/// # Returns
/// A Result containing either the parsed HeuristResponse or an error
pub fn call_heurist_mesh(agent_id: &str, query: &str) -> Result<String> {
    let request_body = HeuristRequestBody {
        agent_id: agent_id.to_string(),
        input: HeuristInput {
            query: query.to_string(),
        },
    };

    // Send the request to the Heurist API
    let response = ureq::post(&HEURIST_MESH_URL)
        .set("Authorization", &format!("Bearer {}", *HEURIST_API_KEY))
        .set("Content-Type", "application/json")
        .send_json(&request_body)
        .map_err(|e| anyhow::anyhow!("Failed to send request to Heurist API: {}", e))?;

    let status = response.status();
    let response_text = response
        .into_string()
        .map_err(|e| anyhow::anyhow!("Failed to read Heurist API response: {}", e))?;

    // Handle response based on status code
    if status >= 200 && status < 300 {
        // Try to parse the response as JSON
        match serde_json::from_str::<HeuristResponse>(&response_text) {
            Ok(parsed) => {
                // If parsing succeeds and response field is not empty, return it
                if !parsed.response.is_empty() {
                    Ok(parsed.response)
                } else {
                    // If response field is empty, return the entire response text
                    Ok(response_text)
                }
            }
            Err(_) => {
                // If parsing fails, return the entire response text
                Ok(response_text)
            }
        }
    } else {
        Err(anyhow::anyhow!(
            "API error (status {}): {}",
            status,
            response_text
        ))
    }
}

pub async fn get_popular_tokens() -> Result<Vec<String>> {
    let msg = call_heurist_mesh(
        "ElfaTwitterIntelligenceAgent",
        r#"{  
        "tool": "get_trending_tokens",  
        "tool_arguments": {"time_window": "24h",},  
        "query": "Get the 16 popular tokens for reference"  
    }"#,
    )?;
    let converted_data = prompt(&format!("{}{}","Extract the Token symbol from the following information and put it into a array (You only give the final data results): ",msg), "qwen/qwen-2.5-72b-instruct").await?;
    // Extract token symbols from the JSON string
    let tokens: Vec<String> = converted_data
        .trim_start_matches("```json\n")
        .trim_end_matches("\n```")
        .trim_start_matches('[')
        .trim_end_matches(']')
        .split(',')
        .map(|s| s.trim().trim_matches('"').to_string().to_uppercase())
        .filter(|token| !EXCLUSION_LIST.contains(&token.as_str()))
        .collect();

    Ok(tokens)
}
#[cfg(test)]
mod tests {
    use super::HeuristResponse;
    use super::*;

    #[test]
    fn test_call_heurist_mesh_basic() -> Result<()> {
        env_logger::init();
        dotenv::dotenv().ok();
        // let response = call_heurist_mesh("TwitterInfoAgent", "What are BeckerC999's last 4 tweets?")?;
        // let response = call_heurist_mesh(
        //     "ElfaTwitterIntelligenceAgent",
        //     r#"{
        //     "tool": "get_trending_tokens",
        //     "tool_arguments": {"time_window": "24h",},
        //     "query": "Get the 16 popular tokens for reference"
        // }"#,
        // )
        // .unwrap();
        // let response = call_heurist_mesh("BitquerySolanaTokenInfoAgent", "get_top_trending_tokens")?;
        // let response = call_heurist_mesh("BitquerySolanaTokenInfoAgent", "Top 20 crypto by market cap")?;
        let response = call_heurist_mesh("BitquerySolanaTokenInfoAgent", "query_token_metrics for 4wZNdqQhxg74Gmmy6YmSJJ2qq4EAz9TUVyCNRGM8pump")?;

        println!("response: {:?}", response);
        // Verify response is not empty
        assert!(!response.is_empty(), "Response text should not be empty");

        Ok(())
    }

    #[test]
    fn test_response_deserialization() {
        // Simple test case with minimal JSON
        let json_str = "{\"response\":\"Test response\",\"data\":{}}";

        let result = serde_json::from_str::<HeuristResponse>(json_str);
        assert!(result.is_ok(), "Should deserialize correctly");

        let response = result.unwrap();
        assert_eq!(response.response, "Test response");
    }

    #[tokio::test]
    async fn test_get_popular_tokens() {
        dotenv::dotenv().ok();
        let tokens = get_popular_tokens().await.unwrap();
        println!("tokens: {:?}", tokens);
    }
}
