use crate::service::data::dex::raydium::{query_raydium_pools, PoolInfo};
use anyhow::Result;
use once_cell::sync::Lazy;
use sled;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

// Define the token lists
const EXCLUSION_LIST: &[&str] = &[
    "SOL", "WSOL", "WBTC", "BTC", "ETH", "WETH", "USDC", "USDT", "RAY",
];
const INCLUSION_LIST: &[&str] = &["SOL", "WSOL", "USDC", "USDT"];

pub static FILTERED_POOLS: Lazy<Arc<Mutex<Vec<PoolInfo>>>> =
    Lazy::new(|| Arc::new(Mutex::new(Vec::with_capacity(100))));

/// Analyze Raydium pools by querying twice with 2s interval and filtering results
pub fn analyze_pools() -> Result<Vec<PoolInfo>> {
    let mut filtered_pools = Vec::new();

    for i in 0..2 {
        // Query Raydium pools
        let response = query_raydium_pools("all", 100 + i, "volume24h", "desc", 16)?;

        // Process each pool in the response
        for pool in response.data.data {
            let symbol_a = &pool.mint_a.symbol.as_str();
            let symbol_b = &pool.mint_b.symbol.as_str();

            // Check if symbols are in exclusion list
            let a_in_exclusion = EXCLUSION_LIST.contains(&symbol_a);
            let b_in_exclusion = EXCLUSION_LIST.contains(&symbol_b);

            // Check if at least one symbol is in inclusion list
            let a_in_inclusion = INCLUSION_LIST.contains(&symbol_a);
            let b_in_inclusion = INCLUSION_LIST.contains(&symbol_b);

            // Apply the filter condition:
            // 1. symbol_a and symbol_b cannot both be in EXCLUSION_LIST (at least one must not be in it)
            // 2. At least one symbol must be in the INCLUSION_LIST
            if !(a_in_exclusion && b_in_exclusion) && (a_in_inclusion || b_in_inclusion) {
                filtered_pools.push(pool);
            }
        }

        // Wait for 2 seconds before next iteration
        thread::sleep(Duration::from_secs(2));
    }
    Ok(filtered_pools)
}

/// Start a service that continuously analyzes pools and updates the FILTERED_POOLS
///
/// This function:
/// 1. Loads existing pool data from sled database into FILTERED_POOLS
/// 2. Starts a loop that calls analyze_pools() every minute
/// 3. Adds new pools to FILTERED_POOLS if they don't already exist
/// 4. Saves the updated FILTERED_POOLS back to sled database
pub fn start_pool_analysis_service() -> Result<()> {
    // Open the sled database
    let db = sled::open("agent_trade_db")?;

    // Load existing data from sled into FILTERED_POOLS if available
    if let Some(existing_data) = db.get("filtered_pools")? {
        if let Ok(pools) = serde_json::from_slice::<Vec<PoolInfo>>(&existing_data) {
            let mut filtered_pools = FILTERED_POOLS.lock().unwrap();
            *filtered_pools = pools;
            println!("Loaded {} pools from database", filtered_pools.len());
        }
    }

    // Clone Arc for the thread
    let filtered_pools_arc = Arc::clone(&FILTERED_POOLS);

    // Start a thread for continuous analysis
    thread::spawn(move || {
        loop {
            match analyze_pools() {
                Ok(new_pools) => {
                    let mut updated = false;

                    // Lock the FILTERED_POOLS for update
                    let mut filtered_pools = filtered_pools_arc.lock().unwrap();

                    // Check each new pool
                    for pool in new_pools {
                        // Check if this pool ID already exists in FILTERED_POOLS
                        let exists = filtered_pools.iter().any(|p| p.id == pool.id);

                        // If not, add it
                        if !exists {
                            println!(
                                "Adding new pool: {} ({}/{})",
                                pool.id, pool.mint_a.symbol, pool.mint_b.symbol
                            );
                            filtered_pools.push(pool);
                            updated = true;
                        }
                    }

                    // If we added new pools, save to sled
                    if updated {
                        if let Ok(json_data) = serde_json::to_vec(&*filtered_pools) {
                            if let Err(e) = db.insert("filtered_pools", json_data) {
                                eprintln!("Error saving to database: {:?}", e);
                            } else {
                                println!("Saved {} pools to database", filtered_pools.len());
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("Error analyzing pools: {:?}", e);
                }
            }

            // Wait for 1 minute before next analysis
            thread::sleep(Duration::from_secs(600));
        }
    });

    Ok(())
}

/// Query filtered pools with pagination
///
/// This function:
/// 1. Returns a paginated subset of FILTERED_POOLS
/// 2. If FILTERED_POOLS is empty, attempts to load data from the sled database
/// 3. Returns both the paginated pools and the total count
///
/// # Arguments
/// * `page_num` - The page number (1-based)
/// * `page_size` - The number of items per page
///
/// # Returns
/// A tuple containing:
/// * `Vec<PoolInfo>` - The paginated pools
/// * `usize` - The total count of pools
pub fn query_filtered_pools(page_num: usize, page_size: usize) -> Result<(Vec<PoolInfo>, usize)> {
    // Lock the FILTERED_POOLS for reading
    let mut filtered_pools = FILTERED_POOLS.lock().unwrap();

    // If FILTERED_POOLS is empty, try to load from sled database
    if filtered_pools.is_empty() {
        // Open the sled database
        let db = sled::open("agent_trade_db")?;

        // Load existing data from sled into FILTERED_POOLS if available
        if let Some(existing_data) = db.get("filtered_pools")? {
            if let Ok(pools) = serde_json::from_slice::<Vec<PoolInfo>>(&existing_data) {
                *filtered_pools = pools;
                println!(
                    "Loaded {} pools from database for query",
                    filtered_pools.len()
                );
            }
        }
    }

    // Calculate total count
    let total_count = filtered_pools.len();

    // Apply pagination
    let start_index = (page_num - 1) * page_size;
    let end_index = std::cmp::min(start_index + page_size, total_count);

    // Create a new Vec with the paginated results by serializing and deserializing each pool
    let mut paginated_pools = Vec::new();
    if start_index < total_count {
        // First, serialize the entire slice we need
        let slice = &filtered_pools[start_index..end_index];
        let json_string = serde_json::to_string(slice)?;

        // Then deserialize it back to get a new Vec
        paginated_pools = serde_json::from_str(&json_string)?;
    }

    Ok((paginated_pools, total_count))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_analyze_pools() {
        // This test will run the actual analysis
        println!("test_analyze_pools");

        // Run the analysis
        match analyze_pools() {
            Ok(pools) => {
                println!("Found {} matching pools", pools.len());
                for (i, pool) in pools.iter().enumerate() {
                    println!(
                        "Pool #{}: {}/{}",
                        i + 1,
                        pool.mint_a.symbol,
                        pool.mint_b.symbol
                    );
                }
            }
            Err(e) => {
                println!("Error analyzing pools: {:?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }

    #[test]
    fn test_start_pool_analysis_service() {
        // This is a simple test to verify the service starts without errors
        println!("test_start_pool_analysis_service");

        match start_pool_analysis_service() {
            Ok(_) => {
                println!("Pool analysis service started successfully");
                // Sleep briefly to let the service run
                thread::sleep(Duration::from_secs(12));

                // Check if FILTERED_POOLS contains data
                let filtered_pools = FILTERED_POOLS.lock().unwrap();
                println!("FILTERED_POOLS contains {} pools", filtered_pools.len());
                println!(
                    "mint_a.symbol: {}",
                    filtered_pools.first().unwrap().mint_a.symbol
                );
            }
            Err(e) => {
                println!("Error starting pool analysis service: {:?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }

    #[test]
    fn test_query_filtered_pools() {
        println!("test_query_filtered_pools");

        // First, ensure we have some data to work with
        // We'll use start_pool_analysis_service to populate FILTERED_POOLS
        match start_pool_analysis_service() {
            Ok(_) => {
                // Sleep briefly to let the service run and populate data
                thread::sleep(Duration::from_secs(5));

                // Now test pagination with different page sizes
                let test_cases = vec![
                    (1, 5),  // First page, 5 items per page
                    (2, 5),  // Second page, 5 items per page
                    (1, 10), // First page, 10 items per page
                    (3, 3),  // Third page, 3 items per page
                ];

                for (page_num, page_size) in test_cases {
                    match query_filtered_pools(page_num, page_size) {
                        Ok((pools, total_count)) => {
                            println!(
                                "Page {}, Size {}: Got {} pools (total: {})",
                                page_num,
                                page_size,
                                pools.len(),
                                total_count
                            );

                            // Verify the number of returned pools is correct
                            let expected_count = std::cmp::min(
                                page_size,
                                if (page_num - 1) * page_size < total_count {
                                    total_count - (page_num - 1) * page_size
                                } else {
                                    0
                                },
                            );

                            assert_eq!(
                                pools.len(),
                                expected_count,
                                "Expected {} pools for page {}, size {}",
                                expected_count,
                                page_num,
                                page_size
                            );

                            // Print some details about the returned pools
                            for (i, pool) in pools.iter().enumerate() {
                                println!(
                                    "  Pool #{}: {}/{} (ID: {})",
                                    i + 1,
                                    pool.mint_a.symbol,
                                    pool.mint_b.symbol,
                                    pool.id
                                );
                            }
                        }
                        Err(e) => {
                            panic!("Error querying filtered pools: {:?}", e);
                        }
                    }
                }
            }
            Err(e) => {
                println!("Error starting pool analysis service: {:?}", e);
                panic!("Test failed with error: {:?}", e);
            }
        }
    }
}
