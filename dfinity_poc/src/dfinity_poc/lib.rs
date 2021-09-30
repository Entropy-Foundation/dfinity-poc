extern crate hex;
extern crate secp256k1;

use ic_cdk_macros::*;
use secp256k1::{Message, PublicKey, Secp256k1, Signature};
use sha256::digest;

static mut MESSAGE: String = String::new();
static mut PUBLIC_KEY: String = String::new();
static mut FLAG: bool = false;
#[update]
fn store_message(pub_key: String, sign: String, msg: String) -> String {
    ic_cdk::print(format!("Public Key ==> {:?}", pub_key));
    ic_cdk::print(format!("Signature ==> {:?}", sign));
    ic_cdk::print(format!("Message ==> {:?}", msg));

    unsafe {
        let msg_hash = digest(&msg);
        ic_cdk::print(format!("Message ==> {:?}", msg_hash));

        let secp = Secp256k1::verification_only();
        let public_key = PublicKey::from_slice(&hex::decode(&pub_key).unwrap())
            .expect("public keys must be 33 or 65 bytes, serialized according to SEC 2");
        let message = Message::from_slice(&hex::decode(&msg_hash).unwrap())
            .expect("messages must be 32 bytes and are expected to be hashes");
        if PUBLIC_KEY.eq(&pub_key) {
            ic_cdk::print(format!("Public Key Authenticated "));
            FLAG = true;
        } else {
            ic_cdk::print(format!("Public Key Not Authenticated "));
            FLAG = false;
        }

        if FLAG {
            let sig = Signature::from_compact(&hex::decode(&sign).unwrap())
                .expect("compact signatures are 64 bytes; DER signatures are 68-72 bytes");

            ic_cdk::print(format!("Signature => {:?}", &sig));
            ic_cdk::print(format!(
                "Verify => {:?}",
                &secp.verify(&message, &sig, &public_key).is_ok()
            ));
            let is_verify: bool = secp.verify(&message, &sig, &public_key).is_ok();

            if is_verify {
                update_message(msg);
                "Signature Verified ,Message Will be Store".to_string()
            } else {
                "Signature not verified , Message not store".to_string()
            }
        } else {
            "Public key not verified , Message not store".to_string()
        }
    }
}

#[update]
fn store_pub_key(pub_key: String, sign: String, msg: String) -> String {
    ic_cdk::print("UPDATING NEW PUBLIC KEY");
    ic_cdk::print(format!("Public Key ==> {:?}", pub_key));
    ic_cdk::print(format!("Signature ==> {:?}", sign));
    ic_cdk::print(format!("Message ==> {:?}", msg));

    unsafe {
        if FLAG {
            let msg_hash = digest(&msg);
            ic_cdk::print(format!("Message ==> {:?}", msg_hash));
            let secp = Secp256k1::verification_only();
            let public_key = PublicKey::from_slice(&hex::decode(&pub_key).unwrap())
                .expect("public keys must be 33 or 65 bytes, serialized according to SEC 2");
            let message = Message::from_slice(&hex::decode(&msg_hash).unwrap())
                .expect("messages must be 32 bytes and are expected to be hashes");
            if PUBLIC_KEY.eq(&pub_key) {
                ic_cdk::print(format!("Public Key Authenticated "));
                FLAG = true;
            } else {
                ic_cdk::print(format!("Public Key Not Authenticated "));
                FLAG = false;
            }
            if FLAG {
                let sig = Signature::from_compact(&hex::decode(&sign).unwrap())
                    .expect("compact signatures are 64 bytes; DER signatures are 68-72 bytes");
                ic_cdk::print(format!("Signature => {:?}", &sig));
                ic_cdk::print(format!(
                    "Verify => {:?}",
                    &secp.verify(&message, &sig, &public_key).is_ok()
                ));
                let is_verify: bool = secp.verify(&message, &sig, &public_key).is_ok();
                if is_verify {
                    // let msg1 = String::from("04").push_str(&msg);
                    let msg1 = format!("{msg}", msg = msg.clone());
                    PUBLIC_KEY.clear();
                    PUBLIC_KEY.push_str(msg1.as_str());
                    "New Public key inserted".to_string()
                } else {
                    "Signature not verified , Message not store".to_string()
                }
            } else {
                "Public key not verified , Message not store".to_string()
            }
        } else {
            "No Public key available".to_string()
        }
    }
}

#[update]
fn update_message(message: String) {
    unsafe {
        MESSAGE.clear();
        MESSAGE.push_str(message.as_str());
    }
}

#[update]
fn update_key(key: String) {
    unsafe {
        if FLAG {
            "Public Key is already there".to_string();
        } else {
            PUBLIC_KEY.clear();
            PUBLIC_KEY.push_str(key.as_str());
        }
    }
}

#[query]
fn get_message() -> String {
    unsafe { MESSAGE.to_string() }
}

#[query]
fn get_public_key() -> String {
    unsafe { PUBLIC_KEY.to_string() }
}

// fn modify_public_key
