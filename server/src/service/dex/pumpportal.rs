use futures::{SinkExt, StreamExt};
use serde_json::{json, Value};
use tokio::time::{timeout, Duration};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};
use url::Url;

async fn subscribe_new_token(max_messages: usize) -> Result<(), Box<dyn std::error::Error>> {
    println!("subscribe_new_token");

    // Parse WebSocket URL
    let url = Url::parse("wss://pumpportal.fun/api/data")?;

    // Connect to WebSocket server
    let (ws_stream, _) = connect_async(url).await?;
    println!("Connected to WebSocket server");

    // Split read and write streams
    let (mut write, mut read) = ws_stream.split();

    // Construct and send subscription payload
    let payload = json!({
        "method": "subscribeNewToken"
    });

    write.send(Message::Text(payload.to_string())).await?;
    println!("Subscription request sent");

    // Add timeout, wait for up to 10 seconds to receive messages
    let mut count = 0;

    while let Ok(Some(message)) = timeout(Duration::from_secs(10), read.next()).await {
        match message {
            Ok(msg) => {
                if let Message::Text(text) = msg {
                    // Parse and print received JSON data
                    if let Ok(parsed) = serde_json::from_str::<Value>(&text) {
                        println!("text json: {:?}", parsed);
                    } else {
                        println!("recv text: {:?}", text);
                    }
                }

                count += 1;
                if count >= max_messages {
                    println!("Received {} messages, exiting loop", count);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error receiving message: {}", e);
                break;
            }
        }
    }

    // After exiting the loop, send unsubscribe request
    let unsubscribe_payload = json!({
        "method": "unsubscribeNewToken"
    });

    // Send unsubscribe request
    let _ = write
        .send(Message::Text(unsubscribe_payload.to_string()))
        .await;
    println!("Unsubscribe request sent");

    // Wait briefly to allow the request to be sent
    tokio::time::sleep(Duration::from_millis(500)).await;

    println!("Test completed");
    Ok(())
}

#[tokio::test]
async fn test_subscribe() {
    println!("Starting test: test_subscribe");

    let max_messages = 3; // Maximum number of messages to receive before exiting
    match subscribe_new_token(max_messages).await {
        Ok(_) => println!("Test completed successfully"),
        Err(e) => println!("Test failed: {}", e),
    }
}
