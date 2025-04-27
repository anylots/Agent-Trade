use std::convert::Infallible;
use std::net::SocketAddr;
use std::time::Duration;

use crate::service::sns::call_heurist_mesh;
use axum::body::Body;
use axum::debug_handler;
use axum::extract::Request;
use axum::middleware::{self, Next};
use axum::response::sse::Event;
use axum::response::{Response, Sse};
use axum::{extract::Json, http::StatusCode, response::IntoResponse};
use axum::{routing::post, Router};
use futures::stream::{self, Stream};
use futures::StreamExt;
use rig::{completion::Prompt, providers::openai};
use serde::{Deserialize, Serialize};
use tower_http::cors::{Any, CorsLayer};

use crate::utils::read_parse_env;

// Request data structure
#[derive(Debug, Deserialize)]
pub struct AgentRequest {
    context: String,
    msg: String,
}

// Response data structure
#[derive(Debug, Serialize)]
pub struct AgentResponse {
    data: String,
    status: String,
}

// Request model
#[derive(Debug, Deserialize)]
struct ChatRequest {
    model: String,
    messages: Vec<Message>,
    stream: bool,
}

#[derive(Debug, Deserialize)]
struct Message {
    role: String,
    content: String,
}

// Response model
#[derive(Debug, Serialize)]
struct ChatResponse {
    id: String,
    choices: Vec<Choice>,
}

#[derive(Debug, Serialize)]
struct Choice {
    index: i32, // Add index field
    delta: Delta,
}

#[derive(Debug, Serialize)]
struct Delta {
    content: String,
}

pub async fn start() {
    let app = create_router();
    // Create CORS middleware
    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = app.layer(cors);

    let addr: SocketAddr = "[::1]:3030".parse().unwrap();
    log::info!("Server running on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn logging_middleware(req: Request, next: Next) -> Response {
    println!("Method: {:?}", req.method());
    println!("URI: {:?}", req.uri());
    println!("Headers: {:?}", req.headers());

    // If request body needs to be inspected
    let (parts, body) = req.into_parts();
    let bytes = axum::body::to_bytes(body, usize::MAX).await.unwrap();
    println!("Body: {:?}", String::from_utf8_lossy(&bytes));

    let req = Request::from_parts(parts, Body::from(bytes));

    next.run(req).await
}

pub fn create_router() -> Router {
    Router::new()
        .route("/agent/prompt", post(handle_agent_prompt))
        .route("/agent/prompt_trade", post(handle_agent_trade))
        .route("/agent/chat/completions", post(chat_stream))
        .layer(middleware::from_fn(logging_middleware))
}

#[debug_handler]
async fn chat_stream(
    Json(payload): Json<ChatRequest>,
) -> Sse<impl Stream<Item = Result<Event, Infallible>>> {
    // If you need to log the request, you can print the payload
    println!("Request payload: {:?}", payload);

    log::info!("process start");

    // Simulated response text
    let fake_response = "# Key Considerations for Building the World's Tallest Structure\n\n\
    When planning the world's tallest building, several core aspects need to be considered:\n\n\
    ## 1. Structural Foundation\n\
        - **Foundation Design**: Must extend into bedrock\n\
        - *Soil Analysis*: Ensure geological stability\n\
        - Seismic Design: Minimum 8.0 magnitude resistance\n\n\
        ";
    // Split response text into chunks
    let chunks: Vec<String> = fake_response
        .lines()
        .map(|line| format!("{}\n", line))
        .collect();

    // Create stream
    let stream = stream::iter(chunks).then(|chunk| async move {
        tokio::time::sleep(Duration::from_millis(50)).await; // Add delay to simulate real effect

        let response = ChatResponse {
            id: "chat-response-id".to_string(),
            choices: vec![Choice {
                index: 0, // Add index value
                delta: Delta { content: chunk },
            }],
        };

        let data = serde_json::to_string(&response).unwrap();

        Ok(Event::default().data(data))
    });
    log::info!("process end");

    // Add [DONE] signal at the end
    let stream = stream.chain(stream::once(async { Ok(Event::default().data("[DONE]")) }));

    Sse::new(stream)
}

// Processing Function
pub async fn handle_agent_trade(
    Json(payload): Json<AgentRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    log::info!("AgentRequest.msg: {}", &payload.msg);
    log::info!("AgentRequest.context: {}", &payload.context);
    // verification
    if !validate_req(&payload.msg) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Call Heurist Mesh API
    match call_heurist_mesh("DexScreenerTokenInfoAgent", &payload.msg) {
        Ok(result) => {
            let response = AgentResponse {
                data: result,
                status: "success".to_string(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Processing Function
pub async fn handle_agent_prompt(
    Json(payload): Json<AgentRequest>,
) -> Result<impl IntoResponse, StatusCode> {
    log::info!("AgentRequest.msg: {}", &payload.msg);
    log::info!("AgentRequest.context: {}", &payload.context);
    // verification
    if !validate_req(&payload.msg) {
        return Err(StatusCode::UNAUTHORIZED);
    }

    // Handling Requests
    match process_agent_request(&payload).await {
        Ok(result) => {
            let response = AgentResponse {
                data: result,
                status: "success".to_string(),
            };
            Ok((StatusCode::OK, Json(response)))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

// Validation Function
fn validate_req(msg: &str) -> bool {
    !msg.is_empty()
}

// Specific logic for processing requests
async fn process_agent_request(
    request: &AgentRequest,
) -> Result<String, Box<dyn std::error::Error>> {
    // let data = prompt_with_agent(&request.context, &request.msg).await;
    let data = format!("prompt_with_agent: {:?}", request.msg);

    Ok(data)
}

async fn prompt_with_agent(context: &str, msg: &str) -> String {
    // Create OpenAI client and model
    let openai_base_api: String = read_parse_env("OPENAI_BASE_API");
    let openai_api_key: String = read_parse_env("OPENAI_API_KEY");
    let model_name: String = read_parse_env("MODEL_NAME");
    let openai_client = openai::Client::from_url(&openai_api_key, &openai_base_api);

    // agent
    let agent = openai_client
        .agent(&model_name)
        .context(context)
        .max_tokens(2048)
        .build();

    // prompt
    agent
        .prompt(msg)
        .await
        .map_err(|e| {
            log::error!("LLM generation failed: {}", e);
            "LLM generation failed"
        })
        .unwrap()
}
