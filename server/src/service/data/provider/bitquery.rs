use reqwest;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::Path;

#[derive(Serialize)]
struct GraphQLQuery {
    query: String,
    variables: Option<serde_json::Value>,
}

const TOTAL_SUPPLY: u64 = 1_000_000_000u64;

async fn fetch_with_market_value_range(
    from: u64,
    to: u64,
) -> Result<(), Box<dyn std::error::Error>> {
    // Constructing GraphQL queries
    // Calculate the price range, convert to 6 decimal places
    let from_price = format!("{:.6}", from as f64 / TOTAL_SUPPLY as f64);
    let to_price = format!("{:.6}", to as f64 / TOTAL_SUPPLY as f64);
    println!("from_price: {:?}, to_price: {:?}", from_price, to_price);

    // Use the original string and then replace only the values ​​you need
    let solana_query = r#"{
      Solana {
        DEXTrades(
          limitBy: {by: Trade_Buy_Currency_MintAddress, count: 1}
          limit: {count: 10}
          orderBy: {descending: Trade_Buy_Price}
          where: {Trade: {Dex: {ProtocolName: {is: "pump_amm"}}, Buy: {Currency: {UpdateAuthority: {is: "TSLvdd1pWpHVjahSpsvCXUbgwsL3JAcvokwaKt1eokM"}}, PriceInUSD: {gt: GT_VALUE, le: LE_VALUE}}, Sell: {AmountInUSD: {gt: "10"}}}, Transaction: {Result: {Success: true}}, Block: {Time: {since: "2025-04-29T08:20:00Z"}}}
        ) {
          Trade {
            Buy {
              Price(maximum: Block_Time)
              PriceInUSD(maximum: Block_Time)
              Currency {
                Name
                Symbol
                MintAddress
                Decimals
                Fungible
                Uri
              }
            }
            Market {
              MarketAddress
            }
          }
          joinTokenSupplyUpdates(
            TokenSupplyUpdate_Currency_MintAddress: Trade_Buy_Currency_MintAddress
            join: inner
            where: {Instruction: {Program: {Address: {is: "6EF8rrecthR5Dkzon8Nwu78hRvfCKubJ14M5uBEwF6P"}, Method: {is: "create"}}}}
          ) {
            Block {
              Time
            }
            Transaction{
              Dev: Signer
              Signature
            }
          }
        }
      }
    }"#
      .replace("GT_VALUE", &from_price)
      .replace("LE_VALUE", &to_price);

    let client = reqwest::Client::new();
    let res = client
        .post("https://streaming.bitquery.io/eap")
        .header("Content-Type", "application/json")
        .header("Authorization", "Bearer xxxx") // 需要添加你的Bitquery API密钥
        .json(&GraphQLQuery {
            query: solana_query,
            variables: None,
        })
        .send()
        .await
        .unwrap();
    // println!("res: {:#?}", res.text().await.unwrap());

    let json_result: serde_json::Value = res.json().await.unwrap();
    println!("{:#?}", json_result);

    // Save JSON results to a file
    let json_str = serde_json::to_string_pretty(&json_result).unwrap();
    fs::write("bitquery_result.json", json_str).unwrap();

    Ok(())
}

#[tokio::test]
async fn test_fetch_with_market_value_range() {
    println!("Start fetch_with_market_value_range...");
    fetch_with_market_value_range(50000, 100000).await.unwrap();
}

#[test]
fn test_load_json_file() {
    // Check if a file exists
    let path = Path::new("bitquery_result.json");
    if !path.exists() {
        println!("File does not exist. Run test_fetch_with_market_value_range first.");
        return;
    }

    // Reading file contents
    let json_str = fs::read_to_string(path).unwrap();

    // Parsing JSON
    let json_result: serde_json::Value = serde_json::from_str(&json_str).unwrap();

    // Print JSON content
    println!("Loaded JSON from file:");
    println!("{:#?}", json_result);
}
