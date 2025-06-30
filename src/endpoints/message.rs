use crate::api_response::ApiResponse;
use axum::Json;
use base64::{engine::general_purpose::STANDARD, Engine};
use bs58;
use ed25519_dalek::{PublicKey as VerifyingKey, Signature, Verifier};
use serde::Deserialize;
use solana_sdk::signer::{keypair::Keypair, Signer};

#[derive(Deserialize)]
pub struct SignRequest {
    pub message: String,
    pub secret_key: String,
}

pub async fn sign_message(Json(req): Json<SignRequest>) -> Json<ApiResponse<String>> {
    let secret_bytes = match bs58::decode(&req.secret_key).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Json(ApiResponse::error("Invalid base58 secret_key")),
    };

    let keypair = match Keypair::from_bytes(&secret_bytes) {
        Ok(kp) => kp,
        Err(_) => return Json(ApiResponse::error("Failed to parse secret key")),
    };

    let signature = keypair.sign_message(req.message.as_bytes());
    let sig_b64 = STANDARD.encode(signature.as_ref());
    Json(ApiResponse::ok(sig_b64))
}

#[derive(Deserialize)]
pub struct VerifyRequest {
    pub message: String,
    pub pubkey: String,
    pub signature: String,
}

pub async fn verify_message(Json(req): Json<VerifyRequest>) -> Json<ApiResponse<bool>> {
    let pubkey_bytes = match bs58::decode(&req.pubkey).into_vec() {
        Ok(bytes) => bytes,
        Err(_) => return Json(ApiResponse::error("Invalid base58 pubkey")),
    };

    if pubkey_bytes.len() != 32 {
        return Json(ApiResponse::error("Invalid pubkey length"));
    }

    let sig_bytes = match STANDARD.decode(&req.signature) {
        Ok(bytes) => bytes,
        Err(_) => return Json(ApiResponse::error("Invalid base64 signature")),
    };

    if sig_bytes.len() != 64 {
        return Json(ApiResponse::error("Invalid signature length"));
    }

    let verifying_key = match VerifyingKey::from_bytes(&pubkey_bytes) {
        Ok(vk) => vk,
        Err(_) => return Json(ApiResponse::error("Invalid public key bytes")),
    };

    let signature = match Signature::from_bytes(&sig_bytes) {
        Ok(sig) => sig,
        Err(_) => return Json(ApiResponse::error("Invalid signature format")),
    };

    let is_valid = verifying_key
        .verify(req.message.as_bytes(), &signature)
        .is_ok();
    Json(ApiResponse::ok(is_valid))
}
