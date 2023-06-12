use crate::update_ic::send_msg;

pub async fn start_vp(canister_id: &str, is_mainnet: bool) -> Result<(), String> {
    let method_name = "start";
    let args = vec![];
    send_msg(canister_id, method_name, args, is_mainnet).await
}

pub async fn restart_vp(canister_id: &str, is_mainnet: bool) -> Result<(), String> {
    let method_name = "restart";
    let args = vec![];
    send_msg(canister_id, method_name, args, is_mainnet).await
}

#[cfg(test)]
mod tests {
    use super::{restart_vp, start_vp};
    const CANISTER_ID: &str = "novnv-7iaaa-aaaaa-aacca-cai";

    #[test]
    fn start_works() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let ret = start_vp(CANISTER_ID, false).await;
            assert!(ret.is_ok());
        });
    }

    #[test]
    fn restart_works() {
        tokio::runtime::Runtime::new().unwrap().block_on(async {
            let ret = restart_vp(CANISTER_ID, false).await;
            match ret {
                Ok(_) => assert!(false),
                Err(e) => {
                    println!("error: {:?}", e.to_string());
                    assert!(e.contains("unauthorized"));
                }
            }
        });
    }
}
