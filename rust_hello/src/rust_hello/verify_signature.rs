extern crate secp256k1;
extern crate hex;

use secp256k1::{Secp256k1, Message, Signature, PublicKey};
// use regex::Regex ;

#[ic_cdk_macros::query]
fn main () {

    let pub_key =hex::decode("046af88181c631af143e9e36ed640b63b322b19be54b7ee2e2c4ba05b54d9bda02b14766da63d6c311479fceb67660ba4f09942d2b41dbd3d05e21ad2283855fe5")
        .expect("Public key Decoding failed");
    ic_cdk::print(format!("Public Key Decoded==> {:?}", pub_key));


    let sign = hex::decode("542e5a6f3063b973d08f6338e944265b1ff833c6373590b973dea0d9d11f2ba161fb1e732225d1ccce1f14fed4bdc02ee4895e91fdb1e204de9aefdfecc4d0da").expect("Signature Decoding failed");
    ic_cdk::print(format!("Signature Decoded==> {:?}", sign));

    // let result = decode_hex("6b2a82b841ca4705acefb6bad81b4333ce87fb3dfccbb8c1be83d497b8cdaf9e22c32b4d525e659ab04c94136b3c0fc886449daf929cdb5307cf5179d487f69d");
    // ic_cdk::print(format!("Signature Decoded==> {:?}", result));

    let secp = Secp256k1::verification_only();



    let public_key = PublicKey::from_slice(&[
        4, 106, 248, 129, 129, 198, 49, 175,
        20, 62, 158, 54, 237, 100, 11, 99,
        179, 34, 177, 155, 229, 75, 126, 226,
        226, 196, 186, 5, 181, 77, 155, 218,
        2, 177, 71, 102, 218, 99, 214, 195,
        17, 71, 159, 206, 182, 118, 96, 186,
        79, 9, 148, 45, 43, 65, 219, 211,
        208, 94, 33, 173, 34, 131, 133, 95, 229]).expect("public keys must be 33 or 65 bytes, serialized according to SEC 2");

    let message = Message::from_slice(&hex::decode("8313615bab4107c0df85238c932fb93647f0bacfdb718ea1699ec6fcb2f359f9").unwrap()).expect("messages must be 32 bytes and are expected to be hashes");

    // println!("{:?}", public_key);
    ic_cdk::print(format!("Public Key => {:?}", public_key));

    ic_cdk::print(format!("Message => {:?}", message));
    let sig =Signature::from_compact(&[
        84, 46, 90, 111, 48, 99, 185, 115,
        208, 143, 99, 56, 233, 68, 38, 91,
        31, 248, 51, 198, 55, 53, 144, 185,
        115, 222, 160, 217, 209, 31, 43, 161,
        97, 251, 30, 115, 34, 37, 209, 204,
        206, 31, 20, 254, 212, 189, 192, 46,
        228, 137, 94, 145, 253, 177, 226, 4,
        222, 154, 239, 223, 236, 196, 208, 218]).expect("compact signatures are 64 bytes; DER signatures are 68-72 bytes");

    ic_cdk::print(format!("Signature => {:?}", &sig));
    ic_cdk::print(format!("Verify => {:?}", &secp.verify(&message, &sig, &public_key).is_ok()));



}

