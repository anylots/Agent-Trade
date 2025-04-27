use std::sync::Arc;

use alloy::{
    network::EthereumWallet,
    providers::{Provider, ProviderBuilder, RootProvider},
    rpc::types::{TransactionReceipt, TransactionRequest},
    signers::local::PrivateKeySigner,
};
use anyhow::anyhow;

use crate::utils;

pub async fn send_eoa_tx(
    request: TransactionRequest,
    provider: RootProvider,
) -> Result<TransactionReceipt, anyhow::Error> {
    // Read the private key from the environment variable
    let private_key = utils::read_parse_env::<String>("EVM_PRIVATE_KEY");
    let signer: PrivateKeySigner = private_key.parse().expect("parse PrivateKeySigner");
    let wallet: EthereumWallet = EthereumWallet::from(signer.clone());

    // Create eth signer.
    let signer = Arc::new(
        ProviderBuilder::new()
            .wallet(wallet)
            .on_provider(provider.clone()),
    );

    let pending_tx = signer.send_transaction(request).await?;

    pending_tx
        .get_receipt()
        .await
        .map_err(|e| anyhow!(e.to_string()))
}
