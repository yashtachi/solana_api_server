use crate::api_response::ApiResponse;
use axum::Json;
use serde::Deserialize;
use solana_sdk::signer::{keypair::Keypair, Signer};
use bs58;
use base64::{engine::general_purpose::STANDARD, Engine};
use ed25519_dalek::{Verifier, Signature, PublicKey as VerifyingKey};

#[derive(Deserialize)]
pub struct SignRequest { message: String, secret: String }

pub async fn sign_message(Json(req): Json<SignRequest>) -> Json<ApiResponse<String>> {
    let sk = bs58::decode(&req.secret).into_vec();
    if sk.is_err() { return Json(ApiResponse::err("Invalid secret")); }
    let kp = Keypair::from_bytes(&sk.unwrap());
    if kp.is_err() { return Json(ApiResponse::err("Bad secret key encoding")); }
    let sig = kp.unwrap().sign_message(req.message.as_bytes());
    Json(ApiResponse::ok(STANDARD.encode(sig.as_ref())))
}

#[derive(Deserialize)]
pub struct VerifyRequest { message: String, pubkey: String, signature: String }

pub async fn verify_message(Json(req): Json<VerifyRequest>) -> Json<ApiResponse<bool>> {
    let pubk_b = bs58::decode(&req.pubkey).into_vec().map_err(|_| "Invalid pubkey");
    let sig_b = STANDARD.decode(&req.signature).map_err(|_| "Invalid signature");
    if pubk_b.is_err() || sig_b.is_err() {
        return Json(ApiResponse::err("Bad pubkey or signature"));
    }
    let vk = VerifyingKey::from_bytes(&pubk_b.unwrap()).map_err(|_| "Invalid pubkey bytes");
    let sig = Signature::from_bytes(&sig_b.unwrap()).map_err(|_| "Bad signature bytes");
    if vk.is_err() || sig.is_err() {
        return Json(ApiResponse::err("Signature or key format error"));
    }
    let valid = vk.unwrap().verify(req.message.as_bytes(), &sig.unwrap()).is_ok();
    Json(ApiResponse::ok(valid))
}
