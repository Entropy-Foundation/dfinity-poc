// use ic_cdk_macros::*;
// use ic_cdk::export::candid;
//
// #[import(canister = "rust_hello")]
// struct HelloCanister;
//
// #[update]
// async fn inter_store_message() -> String {
//     HelloCanister::get_public_key().await
// }
//
// #[update]
// async fn inc() -> () {
//     HelloCanister::inc().await
// }
//
// #[update]
// async fn write(input: candid::Nat) -> () {
//     HelloCanister::write(input).await
// }
