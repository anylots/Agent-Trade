use std::collections::HashMap;
use std::sync::Once;

// Import the necessary modules from the crate
use agent_trade::service::token::solana::{
    AiSignal, AiSignalResponse, AiSignalStats, MemeToken, MemeTokenPriceChanges, MemeTokenResponse,
    PaginationParams,
};

// We'll create our own versions of the functions to test
mod test_functions {
    use super::*;
    use std::fs;
    use std::path::Path;

    // The function we want to test, but using our mockable load function
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
}

// Mock module
mod mock {
    use super::*;
    use lazy_static::lazy_static;
    use std::cell::RefCell;
    use std::sync::Mutex;

    thread_local! {
        // Use thread_local for test isolation
        static LOAD_AI_SIGNALS_IMPL: RefCell<fn() -> Result<Vec<AiSignal>, Box<dyn std::error::Error>>> =
            RefCell::new(mock_load_ai_signals);
        static LOAD_MEME_IMPL: RefCell<fn() -> Result<Vec<MemeToken>, Box<dyn std::error::Error>>> =
            RefCell::new(mock_load_meme);
    }


    // Expose the load functions that use the current implementation
    pub fn load_ai_signals() -> Result<Vec<AiSignal>, Box<dyn std::error::Error>> {
        LOAD_AI_SIGNALS_IMPL.with(|cell| {
            let f = *cell.borrow();
            f()
        })
    }

    pub fn load_meme() -> Result<Vec<MemeToken>, Box<dyn std::error::Error>> {
        LOAD_MEME_IMPL.with(|cell| {
            let f = *cell.borrow();
            f()
        })
    }

    lazy_static! {
        // Mock AI signals data
        pub static ref MOCK_AI_SIGNALS: Mutex<Vec<AiSignal>> = Mutex::new(vec![
            AiSignal {
                id: 1,
                name: "ClineMCP".to_string(),
                symbol: "CLNMCP".to_string(),
                price_change: "+57.52%".to_string(),
                price: "$200K".to_string(),
                volume: "$574/421".to_string(),
                rank: 1,
                time: "23:07:25".to_string(),
                top_percentage: "Top10 98.66%".to_string(),
                avatar: "/avatars/cline.png".to_string(),
                stats: AiSignalStats {
                    entry_price: "$0.0454309".to_string(),
                    market_value: "$543K".to_string(),
                    profit: "$253.7K".to_string(),
                    holders: 168,
                },
                buttons: vec!["0.1".to_string(), "0.5".to_string(), "1".to_string(), "2".to_string()],
                percentages: vec!["25%".to_string(), "50%".to_string(), "75%".to_string(), "100%".to_string()],
            },
            AiSignal {
                id: 2,
                name: "House".to_string(),
                symbol: "HOUSE".to_string(),
                price_change: "-2.51%".to_string(),
                price: "$111K".to_string(),
                volume: "$214/102".to_string(),
                rank: 2,
                time: "23:05:12".to_string(),
                top_percentage: "Top10 35.71%".to_string(),
                avatar: "/avatars/house.png".to_string(),
                stats: AiSignalStats {
                    entry_price: "$0.0000061".to_string(),
                    market_value: "$306.1K".to_string(),
                    profit: "$19.2K".to_string(),
                    holders: 565,
                },
                buttons: vec!["0.1".to_string(), "0.5".to_string(), "1".to_string(), "2".to_string()],
                percentages: vec!["25%".to_string(), "50%".to_string(), "75%".to_string(), "100%".to_string()],
            },
            AiSignal {
                id: 3,
                name: "GEN".to_string(),
                symbol: "GEN".to_string(),
                price_change: "+37.13%".to_string(),
                price: "$1,603.7".to_string(),
                volume: "$2/2".to_string(),
                rank: 3,
                time: "22:57:59".to_string(),
                top_percentage: "Top10 35.71%".to_string(),
                avatar: "/avatars/gen.png".to_string(),
                stats: AiSignalStats {
                    entry_price: "$0.0001336".to_string(),
                    market_value: "$133.9K".to_string(),
                    profit: "$39.9K".to_string(),
                    holders: 745,
                },
                buttons: vec!["0.1".to_string(), "0.5".to_string(), "1".to_string(), "2".to_string()],
                percentages: vec!["25%".to_string(), "50%".to_string(), "75%".to_string(), "100%".to_string()],
            },
            AiSignal {
                id: 4,
                name: "LetsBONK".to_string(),
                symbol: "LETSBONK".to_string(),
                price_change: "-2.91%".to_string(),
                price: "$48.1K".to_string(),
                volume: "$89/89".to_string(),
                rank: 4,
                time: "22:57:27".to_string(),
                top_percentage: "Top10 35.71%".to_string(),
                avatar: "/avatars/letsbonk.png".to_string(),
                stats: AiSignalStats {
                    entry_price: "$0.0000365".to_string(),
                    market_value: "$136.5K".to_string(),
                    profit: "$38.1K".to_string(),
                    holders: 280,
                },
                buttons: vec!["0.1".to_string(), "0.5".to_string(), "1".to_string(), "2".to_string()],
                percentages: vec!["25%".to_string(), "50%".to_string(), "75%".to_string(), "100%".to_string()],
            },
            AiSignal {
                id: 5,
                name: "MvG".to_string(),
                symbol: "MVG".to_string(),
                price_change: "+5.18%".to_string(),
                price: "$73.1K".to_string(),
                volume: "$132/146".to_string(),
                rank: 5,
                time: "22:57:20".to_string(),
                top_percentage: "Top10 35.71%".to_string(),
                avatar: "/avatars/mvg.png".to_string(),
                stats: AiSignalStats {
                    entry_price: "$0.0001609".to_string(),
                    market_value: "$160.9K".to_string(),
                    profit: "$41.2K".to_string(),
                    holders: 870,
                },
                buttons: vec!["0.1".to_string(), "0.5".to_string(), "1".to_string(), "2".to_string()],
                percentages: vec!["25%".to_string(), "50%".to_string(), "75%".to_string(), "100%".to_string()],
            },
        ]);

        // Mock meme tokens data
        pub static ref MOCK_MEME_TOKENS: Mutex<Vec<MemeToken>> = Mutex::new(vec![
            MemeToken {
                id: 1,
                name: "EINSTEIN".to_string(),
                description: "New Einstein RL".to_string(),
                price: "$10.1K".to_string(),
                volume: "$5,674.82".to_string(),
                price_changes: MemeTokenPriceChanges {
                    green: "+42.1%".to_string(),
                    red: "+3.3%".to_string(),
                    yellow: "10%".to_string(),
                    blue: "15%".to_string(),
                    green2: "0%".to_string(),
                },
                time: "4s".to_string(),
                txs: "10 TXs".to_string(),
                holders: "18".to_string(),
                avatar: "/avatars/einstein.png".to_string(),
                category: "新创建".to_string(),
            },
            MemeToken {
                id: 2,
                name: "JWC".to_string(),
                description: "John Wick Coin".to_string(),
                price: "$51.5K".to_string(),
                volume: "$14.7K".to_string(),
                price_changes: MemeTokenPriceChanges {
                    green: "96.8%".to_string(),
                    red: "+76%".to_string(),
                    yellow: "0%".to_string(),
                    blue: "0%".to_string(),
                    green2: "0%".to_string(),
                },
                time: "14h".to_string(),
                txs: "24 TXs".to_string(),
                holders: "98".to_string(),
                avatar: "/avatars/jwc.png".to_string(),
                category: "即将打满".to_string(),
            },
            MemeToken {
                id: 3,
                name: "JAWS".to_string(),
                description: "Jaws TikTok Trend".to_string(),
                price: "$49.2K".to_string(),
                volume: "$95.4K".to_string(),
                price_changes: MemeTokenPriceChanges {
                    green: "100%".to_string(),
                    red: "+27%".to_string(),
                    yellow: "Run".to_string(),
                    blue: "0%".to_string(),
                    green2: "0%".to_string(),
                },
                time: "5min/8h".to_string(),
                txs: "360 TXs".to_string(),
                holders: "1507".to_string(),
                avatar: "/avatars/jaws.png".to_string(),
                category: "已开盘".to_string(),
            },
            MemeToken {
                id: 4,
                name: "Walrus".to_string(),
                description: "WALRUS".to_string(),
                price: "$0".to_string(),
                volume: "$0".to_string(),
                price_changes: MemeTokenPriceChanges {
                    green: "1%".to_string(),
                    red: "+0%".to_string(),
                    yellow: "0%".to_string(),
                    blue: "0%".to_string(),
                    green2: "0%".to_string(),
                },
                time: "4s".to_string(),
                txs: "0 TXs".to_string(),
                holders: "0".to_string(),
                avatar: "/avatars/walrus.png".to_string(),
                category: "新创建".to_string(),
            },
            MemeToken {
                id: 5,
                name: "IEM".to_string(),
                description: "Immortal Elon Musk".to_string(),
                price: "$47.4K".to_string(),
                volume: "$12.6K".to_string(),
                price_changes: MemeTokenPriceChanges {
                    green: "95.2%".to_string(),
                    red: "+20%".to_string(),
                    yellow: "Run".to_string(),
                    blue: "0%".to_string(),
                    green2: "75%".to_string(),
                },
                time: "5h".to_string(),
                txs: "81 TXs".to_string(),
                holders: "236".to_string(),
                avatar: "/avatars/iem.png".to_string(),
                category: "即将打满".to_string(),
            },
        ]);
    }

    // Default mock implementations
    pub fn mock_load_ai_signals() -> Result<Vec<AiSignal>, Box<dyn std::error::Error>> {
        Ok(MOCK_AI_SIGNALS.lock().unwrap().clone())
    }

    pub fn mock_load_meme() -> Result<Vec<MemeToken>, Box<dyn std::error::Error>> {
        Ok(MOCK_MEME_TOKENS.lock().unwrap().clone())
    }
}
