use crate::types::Proofs;
use crate::update_ic::{send_msg, send_msg_for_proofs};

pub async fn conn_open_init(
    canister_id: &str,
    is_mainnet: bool,
    msg: Vec<u8>,
) -> Result<(), String> {
    let method_name = "conn_open_init";
    let args = msg;
    send_msg(canister_id, method_name, args, is_mainnet).await
}

pub async fn conn_open_try(
    canister_id: &str,
    is_mainnet: bool,
    msg: Vec<u8>,
) -> Result<Proofs, String> {
    let method_name = "conn_open_try";
    let args = msg;
    send_msg_for_proofs(canister_id, method_name, args, is_mainnet).await
}

pub async fn conn_open_ack(
    canister_id: &str,
    is_mainnet: bool,
    msg: Vec<u8>,
) -> Result<Proofs, String> {
    let method_name = "conn_open_ack";
    let args = msg;
    send_msg_for_proofs(canister_id, method_name, args, is_mainnet).await
}

pub async fn conn_open_confirm(
    canister_id: &str,
    is_mainnet: bool,
    msg: Vec<u8>,
) -> Result<Proofs, String> {
    let method_name = "conn_open_confirm";
    let args = msg;
    send_msg_for_proofs(canister_id, method_name, args, is_mainnet).await
}
