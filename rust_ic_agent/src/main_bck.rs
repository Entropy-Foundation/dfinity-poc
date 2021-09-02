use ic_agent::{Agent, agent::{http_transport::ReqwestHttpReplicaV2Transport}, ic_types::Principal};
use ic_agent::agent::AgentConfig;

// #[tokio::main]
// async fn main() {
//     let sig = vec![2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141, 134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54];
//     let config = AgentConfig::default();
//     let url = "http://127.0.0.1:8000/".to_string();
//     let transport = ReqwestHttpReplicaV2Transport::create(url).unwrap();
//
//     let agent = Agent::builder().with_transport(transport).build().unwrap();
//
//     let canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai").unwrap();
//     let method_name = "get_public_key".to_string();
//
//     let response = agent.query(&canister_id, method_name).call().await;
//     // let response =
//     //     agent.fetch_root_key().await?;
//     // println!("response: {:?}", response.unwrap());
//     if response.is_ok() {
//         println!("response:{:?}",response.unwrap());
//     } else {
//         let error = response.unwrap_err();
//         println!("error response:{:?}",error);
//     }
// }




use candid::{Encode, Decode, CandidType, Nat};
use serde::Deserialize;

#[derive(CandidType)]
struct Argument {
    pub_key: String,
    sign: String,
    msg: String,
}

#[derive(CandidType, Deserialize)]
struct CreateCanisterResult {
    canister_id: Principal,
}

fn create_identity(){

}

async fn create_a_canister() -> Result<Principal, Box<dyn std::error::Error>> {
    let sig = [2, 29, 21, 35, 7, 198, 183, 43, 14, 208, 65, 139, 14, 112, 205, 128, 231, 245, 41, 91, 141, 134, 245, 114, 45, 63, 82, 19, 251, 210, 57, 79, 54];

    let config = AgentConfig::default();
    let url = "http://127.0.0.1:8000".to_string();
    let transport = ReqwestHttpReplicaV2Transport::create(url).unwrap();

    let agent = Agent::builder()
        .with_transport(transport)
        // .with_identity(sig)
        .build()?;
    // Only do the following call when not contacting the IC main net (e.g. a local emulator).
    // This is important as the main net public key is static and a rogue network could return
    // a different key.
    // If you know the root key ahead of time, you can use `agent.set_root_key(root_key)?;`.
    agent.fetch_root_key().await?;
    let management_canister_id = Principal::from_text("rrkah-fqaaa-aaaaa-aaaaq-cai")?;

    let waiter = garcon::Delay::builder()
        .throttle(std::time::Duration::from_millis(500))
        .timeout(std::time::Duration::from_secs(60 * 5))
        .build();

    // Create a call to the management canister to create a new canister ID,
    // and wait for a result.
    // let response = agent.update(&management_canister_id, "store_message")
    //     .with_arg(&Encode!(&Argument {
    //         pub_key: String::from("046af88181c631af143e9e36ed640b63b322b19be54b7ee2e2c4ba05b54d9bda02b14766da63d6c311479fceb67660ba4f09942d2b41dbd3d05e21ad2283855fe5"),
    //         sign: String::from("542e5a6f3063b973d08f6338e944265b1ff833c6373590b973dea0d9d11f2ba161fb1e732225d1ccce1f14fed4bdc02ee4895e91fdb1e204de9aefdfecc4d0da"),
    //         msg: String::from("542e5a6f3063b973d08f6338e944265b1ff833c6373590b973dea0d9d11f2ba161fb1e732225d1ccce1f14fed4bdc02ee4895e91fdb1e204de9aefdfecc4d0da")
    //     })?)
    //     // .with_arg(vec![
    //     //     "046af88181c631af143e9e36ed640b63b322b19be54b7ee2e2c4ba05b54d9bda02b14766da63d6c311479fceb67660ba4f09942d2b41dbd3d05e21ad2283855fe5",
    //     //     "542e5a6f3063b973d08f6338e944265b1ff833c6373590b973dea0d9d11f2ba161fb1e732225d1ccce1f14fed4bdc02ee4895e91fdb1e204de9aefdfecc4d0da",
    //     //     "542e5a6f3063b973d08f6338e944265b1ff833c6373590b973dea0d9d11f2ba161fb1e732225d1ccce1f14fed4bdc02ee4895e91fdb1e204de9aefdfecc4d0da"
    //     // ])
    //     .call_and_wait(waiter)
    //     .await?;

    let response = agent.query(&management_canister_id, "get_public_key")
        // .with_arg(&Encode!(&Argument { amount: None})?)
        .call()
        .await?;

    let result = Decode!(response.as_slice(), CreateCanisterResult)?;
    let canister_id: Principal = result.canister_id;
    Ok(canister_id)
}

#[tokio::main]
async fn main() {
    let canister_id = create_a_canister().await.unwrap();
    eprintln!("{}", canister_id);
}
