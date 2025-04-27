use anyhow::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::utils::{HEURIST_API_KEY, HEURIST_MESH_URL};

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

#[cfg(test)]
mod tests {
    use super::HeuristResponse;
    use super::*;

    #[test]
    fn test_call_heurist_mesh_basic() -> Result<()> {
        env_logger::init();
        dotenv::dotenv().ok();
        let response = call_heurist_mesh("TwitterInfoAgent", "BeckerC999的最新4条推文是什么")?;

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
}
