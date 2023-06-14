use tendermint_vp_sdk::{chan_msg::*, client_msg::*, conn_msg::*, query_ic::*, start_msg::*};
mod common;

use common::data::*;
use std::fs;
const PATH: &str = "tests/resource/canister_id";

#[test]
fn test() {
    let canister_id = fs::read_to_string(PATH).expect("Read file error");
    tokio::runtime::Runtime::new().unwrap().block_on(async {
        let raw_create_client = get_ibc0_create_client();
        let ret = start_vp(&canister_id, false).await;
        assert!(ret.is_ok());

        let ret = create_client(&canister_id, false, raw_create_client).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc0_update_client1();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc1_update_client2();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_connection_open_try = get_ibc1_connection_open_try();
        let ret = conn_open_try(&canister_id, false, raw_connection_open_try).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc1_update_client3();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_connection_open_confirm = get_ibc1_connection_open_confirm();
        let ret = conn_open_confirm(&canister_id, false, raw_connection_open_confirm).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc1_update_client4();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_channel_open_try = get_ibc1_channel_open_try();
        let ret = chan_open_try(&canister_id, false, raw_channel_open_try).await;
        assert!(ret.is_ok());

        let raw_update_client = get_ibc1_update_client5();
        let ret = update_client(&canister_id, false, raw_update_client).await;
        assert!(ret.is_ok());

        let raw_chann_open_confirm = get_ibc1_channel_open_confirm();
        let ret = chan_open_confirm(&canister_id, false, raw_chann_open_confirm).await;
        assert!(ret.is_ok());

        let ret = query_sequence_times(&canister_id, false, 1).await;
        assert!(ret.is_ok());
    });
}
