use ic_agent::{Agent, agent::{http_transport::ReqwestHttpReplicaV2Transport}, ic_types::Principal};
use ic_agent::agent::AgentConfig;
use candid::{Encode, Decode, CandidType, Nat};
use std::str;


async fn update_public_key(){
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
    let response = agent.update(&canister_id, method_name)
        .with_arg(Encode!(&"04a3fe01e1c6ab5306130d09c1a928bd1598ccce020503ade24d0a5bf7040d5f4cdec9fdcba6497f834641b7908ed04d0b7698bbce6100ff2bbf82e5c52d523b19").unwrap()).call_and_wait(waiter).await;
    if response.is_ok() {
        println!("response:{:?}", "Key Updated");
    } else {
        let error = response.unwrap_err();
        println!("error response:{:?}",error);
    }

}

async fn get_public_key() {
    let config = AgentConfig::default();
    let url = "http://127.0.0.1:8000/".to_string();
    let transport = ReqwestHttpReplicaV2Transport::create(url).unwrap();

    let agent = Agent::builder().with_transport(transport).build().unwrap();

    let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
    let method_name = "get_public_key".to_string();

    let response = agent.query(&canister_id, method_name)
        .with_arg(candid::Encode!().unwrap())
        .call().await;
    if response.is_ok() {
        // let res = String::from_utf8(response.unwrap()).unwrap();
        let res = Decode!(&response.unwrap(), String).unwrap();
        println!("response:{:?}", res);

    } else {
        let error = response.unwrap_err();
        println!("error response:{:?}",error);
    }
}

async fn store_message(){
    let sig = vec![2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141, 134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54];
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
    let response = agent.update(&canister_id, method_name)
        .with_arg(Encode!(&"04a3fe01e1c6ab5306130d09c1a928bd1598ccce020503ade24d0a5bf7040d5f4cdec9fdcba6497f834641b7908ed04d0b7698bbce6100ff2bbf82e5c52d523b19",
            &"e368f0ab2a13804e63dbc64d4c25175f117d1a5cb2444416f557423730f9da26678d224e7c6952eea2b99dff14546538b8d3dfb4abe37177b85d0abaa6677935",
            &"webelight").unwrap()).call_and_wait(waiter).await;
    if response.is_ok() {
        let res = Decode!(&response.unwrap(), String).unwrap();
        println!("response:{:?}", res);
    } else {
        let error = response.unwrap_err();
        println!("error response:{:?}",error);
    }
}

async fn update_old_public_key(){
    let sig = vec![2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141, 134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54];
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
    let response = agent.update(&canister_id, method_name)
        .with_arg(Encode!(&"04a3fe01e1c6ab5306130d09c1a928bd1598ccce020503ade24d0a5bf7040d5f4cdec9fdcba6497f834641b7908ed04d0b7698bbce6100ff2bbf82e5c52d523b19",
            &"e7bbc583aa20eee113a2fc79d510e8ba41d74b8913f38b78b5a5c3d6cd12f56d2208e0b60a0ec00d801d11cd143b862b2da4682ba43a32496346d17a35bf08e5",
            &"d2c8c1a98402a276b0e93e061daef33738b7fb372fa142283744bcc6fc7701e4b94556d347841122f696350b34cdfa787f0f32b97255b90707d15b0d1a700cb4").unwrap()).call_and_wait(waiter).await;
    if response.is_ok() {
        let res = Decode!(&response.unwrap(), String).unwrap();
        println!("response:{:?}", res);
    } else {
        let error = response.unwrap_err();
        println!("error response:{:?}",error);
    }
}
#[tokio::main]
async fn main() {
    update_public_key().await;
    get_public_key().await;
    store_message().await;
    update_old_public_key().await;
    get_public_key().await;
    store_message().await;
}
