use candid::{CandidType, Decode, Encode, Nat};
use ic_agent::agent::AgentConfig;
use ic_agent::{agent::http_transport::ReqwestHttpReplicaV2Transport, ic_types::Principal, Agent};
use std::str;

mod dkg;

async fn update_public_key() {
    let config = AgentConfig::default();

    let url = "http://127.0.0.1:8000/".to_string();
    let transport = ReqwestHttpReplicaV2Transport::create(url).unwrap();
    let waiter = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build();
    let agent = Agent::builder().with_transport(transport).build().unwrap();
    agent.fetch_root_key().await;

    let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let method_name = "update_key".to_string();
    let dkg_pub_key = dkg::get_dkg_pub_key().await;

    let response = agent
        .update(&canister_id, method_name)
        .with_arg(Encode!(&dkg_pub_key).unwrap())
        .call_and_wait(waiter)
        .await;

    if response.is_ok() {
        println!("response:{:?}", "Key Updated");
    } else {
        let error = response.unwrap_err();
        println!("error response:{:?}", error);
    }
}

async fn get_public_key() {
    let config = AgentConfig::default();
    let url = "http://127.0.0.1:8000/".to_string();
    let transport = ReqwestHttpReplicaV2Transport::create(url).unwrap();

    let agent = Agent::builder().with_transport(transport).build().unwrap();

    let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let method_name = "get_public_key".to_string();

    let response = agent
        .query(&canister_id, method_name)
        .with_arg(candid::Encode!().unwrap())
        .call()
        .await;
    if response.is_ok() {
        // let res = String::from_utf8(response.unwrap()).unwrap();
        let res = Decode!(&response.unwrap(), String).unwrap();
        println!("response:{:?}", res);
    } else {
        let error = response.unwrap_err();
        println!("error response:{:?}", error);
    }
}

async fn store_message() {
    // let sig = vec![2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141, 134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54];
    let config = AgentConfig::default();

    let url = "http://127.0.0.1:8000/".to_string();
    let waiter = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build();
    let transport = ReqwestHttpReplicaV2Transport::create(url).unwrap();
    let agent = Agent::builder().with_transport(transport).build().unwrap();
    agent.fetch_root_key().await;

    let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let method_name = "store_message".to_string();
    let dkg_pub_key = dkg::get_dkg_pub_key().await;
    let msg: String = "testing-dkg".to_string();
    let dkg_sign = dkg::sign_message(msg.clone()).await;

    let response = agent
        .update(&canister_id, method_name)
        .with_arg(Encode!(&dkg_pub_key, &dkg_sign, &msg).unwrap())
        .call_and_wait(waiter)
        .await;

    if response.is_ok() {
        let res = Decode!(&response.unwrap(), String).unwrap();
        println!("response:{:?}", res);
    } else {
        let error = response.unwrap_err();
        println!("error response:{:?}", error);
    }
}

async fn update_old_public_key() {
    // let sig = vec![2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141, 134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54];
    let config = AgentConfig::default();

    let url = "http://127.0.0.1:8000/".to_string();
    let waiter = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build();
    let transport = ReqwestHttpReplicaV2Transport::create(url).unwrap();
    let agent = Agent::builder().with_transport(transport).build().unwrap();
    agent.fetch_root_key().await;

    let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let method_name = "store_pub_key".to_string();
    let dkg_pub_key = dkg::get_dkg_pub_key().await;
    // let msg: String = dkg::get_dkg_pub_key().await; //TODO:: to add updated pub-key from dkg
    let msg: String = "04a3fe01e1c6ab5306130d09c1a928bd1598ccce020503ade24d0a5bf7040d5f4cdec9fdcba6497f834641b7908ed04d0b7698bbce6100ff2bbf82e5c52d523b19".to_string();
    let dkg_sign = dkg::sign_message(msg.clone()).await;

    let response = agent
        .update(&canister_id, method_name)
        .with_arg(Encode!(&dkg_pub_key, &dkg_sign, &msg).unwrap())
        .call_and_wait(waiter)
        .await;

    if response.is_ok() {
        let res = Decode!(&response.unwrap(), String).unwrap();
        println!("response:{:?}", res);
    } else {
        let error = response.unwrap_err();
        println!("error response:{:?}", error);
    }
}
#[tokio::main]
async fn main() {
    // let msg: String = "testing-dkg".to_string();

    // dkg::get_dkg_pub_key().await;
    // dkg::sign_message(msg).await;

    update_public_key().await;
    get_public_key().await;
    store_message().await;
    update_old_public_key().await;
    get_public_key().await;
    store_message().await;
}
