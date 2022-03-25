use crate::contract::*;
use crate::{address_from_str, u256_from_str, EthError};
use ethers::prelude::{Http, Provider};
use std::sync::Arc;
pub async fn get_name(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let contract_address = address_from_str(contract_address)?;
    let contract = Erc721Contract::new(contract_address, Arc::new(client));
    contract
        .name()
        .call()
        .await
        .map_err(|_| EthError::ContractError)
}

pub async fn get_symbol(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let contract_address = address_from_str(contract_address)?;
    let contract = Erc721Contract::new(contract_address, Arc::new(client));
    contract
        .symbol()
        .call()
        .await
        .map_err(|_| EthError::ContractError)
}

pub async fn get_token_uri(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<String, EthError> {
    let client = Provider::<Http>::try_from(web3api_url).map_err(|_| EthError::NodeUrl)?;
    let contract_address = address_from_str(contract_address)?;
    let contract = Erc721Contract::new(contract_address, Arc::new(client));
    let token_id = u256_from_str(token_id)?;
    contract
        .token_uri(token_id)
        .call()
        .await
        .map_err(|_| EthError::ContractError)
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_name_blocking(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_name(contract_address, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_symbol_blocking(contract_address: &str, web3api_url: &str) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_symbol(contract_address, web3api_url))
}

#[cfg(not(target_arch = "wasm32"))]
pub fn get_token_uri_blocking(
    contract_address: &str,
    token_id: &str,
    web3api_url: &str,
) -> Result<String, EthError> {
    let rt = tokio::runtime::Runtime::new().map_err(|_err| EthError::AsyncRuntimeError)?;
    rt.block_on(get_token_uri(contract_address, token_id, web3api_url))
}