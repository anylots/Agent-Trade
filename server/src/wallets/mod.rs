use std::env;

use alloy::{
    providers::RootProvider,
    rpc::types::{TransactionReceipt, TransactionRequest},
};
use anyhow::anyhow;
use eip7702::send_7702_tx;
use local::send_eoa_tx;
use once_cell::sync::Lazy;

pub mod eip7702;
pub mod local;

pub static ACCONT_TYPE: Lazy<String> = Lazy::new(|| env::var("ACCONT_TYPE").unwrap());

pub async fn send_tx(
    request: TransactionRequest,
    provider: RootProvider,
) -> Result<TransactionReceipt, anyhow::Error> {
    match ACCONT_TYPE.as_str() {
        "EIP7702" => send_7702_tx(request, provider).await,
        "LOCAL" => send_eoa_tx(request, provider).await,
        _ => Err(anyhow!("unknown account type")),
    }
}
