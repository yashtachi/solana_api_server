use crate::api_response::ApiResponse;
use axum::Json;
use bs58;
use serde::Serialize;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;

#[derive(Serialize)]
pub struct KeypairResponse {
    pub pubkey: String,
    pub secret: String,
}

pub async fn generate_keypair() -> Json<ApiResponse<KeypairResponse>> {
    let kp = Keypair::new();
    let pubkey = kp.pubkey().to_string();
    let secret = bs58::encode(kp.to_bytes()).into_string();
    Json(ApiResponse::ok(KeypairResponse { pubkey, secret }))
}
