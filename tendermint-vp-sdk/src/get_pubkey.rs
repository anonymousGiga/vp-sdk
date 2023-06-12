use crate::update_ic::send_msg_for_vec;

pub async fn get_pubkey(canister_id: &str, is_mainnet: bool) -> Result<Vec<u8>, String> {
    let method_name = "public_key";
    let args = vec![];
    send_msg_for_vec(canister_id, method_name, args, is_mainnet)
        .await
        .map_err(|e| e.to_string())
}

#[cfg(test)]
mod tests {
    use super::get_pubkey;
    const CANISTER_ID: &str = "novnv-7iaaa-aaaaa-aacca-cai";

    #[test]
    fn query_pubkey_works() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let ret = get_pubkey(CANISTER_ID, false).await;
            assert!(ret.is_ok());
        });
    }
}
