use reqwest;
use serde::*;
use serde_json::{self, Value};

#[derive(Serialize, Deserialize, Debug)]
struct ReqData {
    id: u32,
    jsonrpc: String,
    method: String,
    params: Param,
}

#[derive(Serialize, Deserialize, Debug)]
struct ReqData1 {
    id: u32,
    jsonrpc: String,
    method: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Param {
    message: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct ResData {
    id: u32,
    jsonrpc: String,
    result: String,
}

pub async fn sign_message(msg: String) -> String {
    let param = Param { message: msg };

    let data = ReqData {
        id: 123,
        jsonrpc: "2.0".into(),
        method: "sign_message".into(),
        params: param,
    };

    let response = reqwest::Client::new()
        .post("http://127.0.0.1:3031")
        .json(&data)
        .send()
        .await;

    let js = response.unwrap().json::<ResData>().await;
    // println!("sign_message response {:#?}", &js.unwrap())

    format!("{}", js.unwrap().result)
}

pub async fn get_dkg_pub_key() -> String {
    let data = ReqData1 {
        id: 123,
        jsonrpc: "2.0".to_string(),
        method: "get_public_key".to_string(),
    };

    let response = reqwest::Client::new()
        .post("http://127.0.0.1:3031")
        .json(&data)
        .send()
        .await;

    let js = response.unwrap().json::<ResData>().await;
    // println!("get_dkg_pub_key response {:#?}", js.unwrap());

    format!("04{}", js.unwrap().result)
}
