use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::service::data::analyze::query_filtered_pools;

const TOKEN_IN_LIST: &[&str] = &["SOL", "WSOL", "USDC", "USDT"];

// Define the structures for AI Signals
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiSignalStats {
    #[serde(rename = "entryPrice")]
    pub entry_price: String,
    #[serde(rename = "marketValue")]
    pub market_value: String,
    pub volume: String,
    pub holders: u32,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AiSignal {
    pub id: u32,
    pub name: String,
    pub symbol: String,
    #[serde(rename = "tokenAddress")]
    pub token_address: String,
    #[serde(rename = "logoUri")]
    pub logo_uri: String,
    #[serde(rename = "priceChange")]
    pub price_change: String,
    pub price: String,
    pub volume: String,
    pub rank: u32,
    pub time: String,
    #[serde(rename = "topPercentage")]
    pub top_percentage: String,
    pub avatar: String,
    pub stats: AiSignalStats,
}

// Define the structures for Meme Tokens
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemeTokenPriceChanges {
    pub green: String,
    pub red: String,
    pub yellow: String,
    pub blue: String,
    pub green2: String,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MemeToken {
    pub id: u32,
    pub name: String,
    pub description: String,
    pub price: String,
    pub volume: String,
    #[serde(rename = "priceChanges")]
    pub price_changes: MemeTokenPriceChanges,
    pub time: String,
    pub txs: String,
    pub holders: String,
    pub avatar: String,
    pub category: String,
}

// Define the query parameters structure
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct PaginationParams {
    pub page_num: usize,
    pub page_size: usize,
    pub extend_param: HashMap<String, String>,
}

// Define the response structure
#[derive(Debug, Serialize, Deserialize)]
pub struct AiSignalResponse {
    pub list: Vec<AiSignal>,
    pub extend_data: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MemeTokenResponse {
    pub list: Vec<MemeToken>,
    pub extend_data: HashMap<String, String>,
}

// Helper function to load the AiSignal data
fn load_ai_signals() -> Result<Vec<AiSignal>, Box<dyn std::error::Error>> {
    let data_path = Path::new("src/service/token/solana-ai-signal.json");
    let data_str = fs::read_to_string(data_path)?;
    let data: Vec<AiSignal> = serde_json::from_str(&data_str)?;
    Ok(data)
}

fn load_ai_signals_from_pool(
    params: PaginationParams,
) -> Result<Vec<AiSignal>, Box<dyn std::error::Error>> {
    let (pools, _) = query_filtered_pools(params.page_num, params.page_size)?;

    // Convert pools to AiSignal format
    let mut data: Vec<AiSignal> = Vec::with_capacity(pools.len());

    for (index, pool) in pools.iter().enumerate() {
        let dis_token = if TOKEN_IN_LIST.contains(&pool.mint_a.symbol.as_str()) {
            &pool.mint_b
        } else {
            &pool.mint_a
        };
        // Create a new AiSignal from pool data
        let signal = AiSignal {
            // Basic identification
            id: (params.page_num * params.page_size + index + 1) as u32, // Use index+1 as ID
            name: dis_token.name.clone(),
            symbol: dis_token.symbol.clone(),
            token_address: dis_token.address.clone(),
            logo_uri: dis_token.logo_uri.clone().unwrap_or_default(),
            // Price information
            price_change: format!("+{:.2}%", pool.day.apr), // Use APR as price change
            price: format!("${:.2}", pool.price),           // Format pool price
            volume: format!("${:.0}/{:.0}", pool.day.volume, pool.week.volume), // Daily/weekly volume

            // Ranking and timing
            rank: (index + 1) as u32,     // Use index+1 as rank
            time: "23:07:25".to_string(), // Use fixed time as in example

            // Additional metrics
            top_percentage: format!(
                "Top{} {:.2}%",
                if index < 10 { "10" } else { "50" },
                (100.0 - (index as f64 * 1.5))
            ), // Generate top percentage
            avatar: format!("/avatars/{}.png", pool.mint_a.symbol.to_lowercase()), // Generate avatar path

            // Financial statistics
            stats: AiSignalStats {
                entry_price: format!("${:.7}", pool.price / 100.0), // Entry price as fraction of current
                market_value: format!("${:.0}K", pool.tvl / 1000.0), // TVL in thousands
                volume: format!("${:.1}K", pool.day.volume / 1000.0), // Daily volume in thousands
                holders: 100 + (index * 10) as u32,                 // Generate holder count
            },
        };

        data.push(signal);
    }

    Ok(data)
}

// Helper function to load the MemeToken data
fn load_meme() -> Result<Vec<MemeToken>, Box<dyn std::error::Error>> {
    let data_path = Path::new("src/service/token/solana-meme.json");
    let data_str = fs::read_to_string(data_path)?;
    let data: Vec<MemeToken> = serde_json::from_str(&data_str)?;
    Ok(data)
}

pub async fn get_ai_signals_paginated(
    params: PaginationParams,
) -> Result<AiSignalResponse, Box<dyn std::error::Error>> {
    // Try to load AI signals from pool first
    let pool_params = PaginationParams {
        page_num: params.page_num,
        page_size: params.page_size,
        extend_param: HashMap::new(),
    };

    let data = match load_ai_signals_from_pool(pool_params) {
        Ok(pool_data) => {
            if !pool_data.is_empty() {
                // If we successfully got data from pools, use it
                pool_data
            } else {
                // Fallback to static data if pool data is empty
                load_ai_signals()?
            }
        }
        Err(_) => {
            // Fallback to static data if there was an error loading from pools
            load_ai_signals()?
        }
    };

    // Create the response with the data
    let response = AiSignalResponse {
        list: data,
        extend_data: HashMap::new(), // Initialize with empty HashMap, can be populated as needed
    };

    Ok(response)
}

pub async fn get_meme_tokens_paginated(
    params: PaginationParams,
) -> Result<MemeTokenResponse, Box<dyn std::error::Error>> {
    let data = load_meme()?;

    // Calculate pagination
    let start_index = (params.page_num - 1) * params.page_size;
    let end_index = std::cmp::min(start_index + params.page_size, data.len());

    // Get the paginated list
    let paginated_list = if start_index < data.len() {
        data[start_index..end_index].to_vec()
    } else {
        Vec::new()
    };

    // Create the response
    let response = MemeTokenResponse {
        list: paginated_list,
        extend_data: HashMap::new(), // Initialize with empty HashMap, can be populated as needed
    };

    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;
    use tokio::runtime::Runtime;

    #[test]
    fn test_get_ai_signals_paginated() {
        // Create a runtime for testing async functions
        let rt = Runtime::new().unwrap();

        // Test case 1: Normal pagination (first page)
        let params1 = PaginationParams {
            page_num: 1,
            page_size: 2,
            extend_param: HashMap::new(),
        };

        let result1 = rt.block_on(get_ai_signals_paginated(params1));
        assert!(result1.is_ok(), "Failed to get first page");
        let response1 = result1.unwrap();
        assert_eq!(response1.list.len(), 2, "First page should have 2 items");

        // // Test case 2: Normal pagination (second page)
        // let params2 = PaginationParams {
        //     page_num: 2,
        //     page_size: 2,
        //     extend_param: HashMap::new(),
        // };

        // let result2 = rt.block_on(get_ai_signals_paginated(params2));
        // assert!(result2.is_ok(), "Failed to get second page");
        // let response2 = result2.unwrap();
        // // Note: This assertion might fail if there are fewer than 4 items in the data
        // // In a real test, you might want to check the actual data size first

        // // Test case 3: Page beyond data range
        // let params3 = PaginationParams {
        //     page_num: 1000,
        //     page_size: 10,
        //     extend_param: HashMap::new(),
        // };

        // let result3 = rt.block_on(get_ai_signals_paginated(params3));
        // assert!(result3.is_ok(), "Failed to handle out-of-range page");
        // let response3 = result3.unwrap();
        // assert_eq!(
        //     response3.list.len(),
        //     0,
        //     "Out-of-range page should return empty list"
        // );

        // // Test case 4: Zero page size
        // let params4 = PaginationParams {
        //     page_num: 1,
        //     page_size: 0,
        //     extend_param: HashMap::new(),
        // };

        // let result4 = rt.block_on(get_ai_signals_paginated(params4));
        // assert!(result4.is_ok(), "Failed to handle zero page size");
        // let response4 = result4.unwrap();
        // assert_eq!(
        //     response4.list.len(),
        //     0,
        //     "Zero page size should return empty list"
        // );
    }

    #[test]
    fn test_get_meme_tokens_paginated() {
        // Create a runtime for testing async functions
        let rt = Runtime::new().unwrap();

        // Test case 1: Normal pagination (first page)
        let params1 = PaginationParams {
            page_num: 1,
            page_size: 2,
            extend_param: HashMap::new(),
        };

        let result1 = rt.block_on(get_meme_tokens_paginated(params1));
        assert!(result1.is_ok(), "Failed to get first page");
        let response1 = result1.unwrap();
        assert_eq!(response1.list.len(), 2, "First page should have 2 items");
    }
}
