use crate::types::*;
use candid::{decode_args, CandidType, Decode, Encode, Nat};
use ic_cdk::export::{
    serde::{Deserialize, Serialize},
    Principal,
};

async fn update_ic(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let url = if is_mainnet { MAIN_NET } else { LOCAL_NET };
    let agent = ic_agent::Agent::builder()
        .with_url(url)
        .build()
        .expect("should work");

    if !is_mainnet {
        agent.fetch_root_key().await?;
    }

    let canister_id = ic_cdk::export::Principal::from_text(canister_id)?;

    let mut update_builder =
        ic_agent::agent::UpdateBuilder::new(&agent, canister_id, method_name.to_string());
    let update_builder_with_args = update_builder.with_arg(&Encode!(&args)?);

    let response = update_builder_with_args.call_and_wait().await?;
    Ok(response)
}

async fn update_ic_and_get_smheader(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<SmHeaderResult, Box<dyn std::error::Error>> {
    let response = update_ic(canister_id, method_name, args, is_mainnet).await?;
    let response = Decode!(response.as_slice(), SmHeaderResult)?;

    Ok(response)
}

async fn update_ic_and_get_nothing(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<NullResult, Box<dyn std::error::Error>> {
    let response = update_ic(canister_id, method_name, args, is_mainnet).await?;
    let response = Decode!(response.as_slice(), NullResult)?;

    Ok(response)
}

async fn update_ic_and_get_smstate(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<SmStateResult, Box<dyn std::error::Error>> {
    let response = update_ic(canister_id, method_name, args, is_mainnet).await?;
    let response = Decode!(response.as_slice(), SmStateResult)?;

    Ok(response)
}

async fn update_ic_and_get_proofs(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<ProofsResult, Box<dyn std::error::Error>> {
    let response = update_ic(canister_id, method_name, args, is_mainnet).await?;
    let response = Decode!(response.as_slice(), ProofsResult)?;

    Ok(response)
}

pub async fn send_msg_for_smheader (
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let result = update_ic_and_get_smheader(canister_id, method_name, args, is_mainnet).await
        .map_err(|e| e.to_string())?;

    match result {
        SmHeaderResult::Ok(smheader) => Ok(smheader),
        SmHeaderResult::Err(e) => Err(e.into()),
    }
}

pub async fn send_msg_for_smstate(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<SmState, Box<dyn std::error::Error>> {
    let result = update_ic_and_get_smstate(canister_id, method_name, args, is_mainnet).await
        .map_err(|e| e.to_string())?;

    match result {
        SmStateResult::Ok(state) => Ok(state),
        SmStateResult::Err(e) => Err(e.into()),
    }
}

pub async fn send_msg_for_proofs(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<Proofs, String> {
    let result = update_ic_and_get_proofs(canister_id, method_name, args, is_mainnet)
        .await
        .map_err(|e| e.to_string())?;

    match result {
        ProofsResult::Ok(proofs) => Ok(proofs),
        ProofsResult::Err(e) => Err(e),
    }
}

pub async fn send_msg(
    canister_id: &str,
    method_name: &str,
    args: Vec<u8>,
    is_mainnet: bool,
) -> Result<(), Box<dyn std::error::Error>> {
    let result = update_ic_and_get_nothing(canister_id, method_name, args, is_mainnet).await
        .map_err(|e| e.to_string())?;

    match result {
        NullResult::Ok(_) => Ok(()),
        NullResult::Err(e) => Err(e.into()),
    }
}