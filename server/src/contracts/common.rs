use crate::chains::get_chain_info;
use alloy::providers::RootProvider;

pub fn make_provider(chain_name: String) -> std::result::Result<RootProvider, anyhow::Error> {
    let provider_url = get_chain_info(&chain_name).unwrap().provider_url;
    let provider = RootProvider::new_http(provider_url.parse()?);
    Ok(provider)
}
