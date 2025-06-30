use crate::api_response::ApiResponse;
use axum::Json;
use solana_sdk::signer::keypair::Keypair;
use solana_sdk::signer::Signer;
use bs58;

pub async fn generate_keypair() -> Json<ApiResponse<(String, String)>> {
    let kp = Keypair::new();
    let pubkey = kp.pubkey().to_string();
    let secret = bs58::encode(kp.to_bytes()).into_string();
    Json(ApiResponse::ok((pubkey, secret)))
}
