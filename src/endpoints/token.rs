use crate::api_response::ApiResponse;
use axum::Json;
use base64::{engine::general_purpose, Engine};
use bincode;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::{initialize_mint, mint_to};
use std::str::FromStr;

#[derive(Deserialize)]
pub struct CreateMint {
    pub mint_pubkey: String,
    pub mint_authority: String,
    pub decimals: u8,
}

pub async fn init_mint(Json(req): Json<CreateMint>) -> Json<ApiResponse<String>> {
    let mint = match Pubkey::from_str(&req.mint_pubkey) {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid base58 mint_pubkey")),
    };

    let auth = match Pubkey::from_str(&req.mint_authority) {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid base58 mint_authority")),
    };

    let ix = match initialize_mint(&spl_token::id(), &mint, &auth, None, req.decimals) {
        Ok(instruction) => instruction,
        Err(e) => {
            return Json(ApiResponse::error(&format!(
                "Failed to initialize mint: {}",
                e
            )))
        }
    };

    let serialized = bincode::serialize(&ix).unwrap();
    let data_b64 = general_purpose::STANDARD.encode(serialized);
    Json(ApiResponse::ok(data_b64))
}

#[derive(Deserialize)]
pub struct MintRequest {
    pub mint_pubkey: String,
    pub destination: String,
    pub authority: String,
    pub amount: u64,
}

pub async fn mint_tokens(Json(req): Json<MintRequest>) -> Json<ApiResponse<String>> {
    let mint = match Pubkey::from_str(&req.mint_pubkey) {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid base58 mint_pubkey")),
    };

    let dest = match Pubkey::from_str(&req.destination) {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid base58 destination")),
    };

    let auth = match Pubkey::from_str(&req.authority) {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid base58 authority")),
    };

    let ix = match mint_to(&spl_token::id(), &mint, &dest, &auth, &[], req.amount) {
        Ok(instruction) => instruction,
        Err(e) => {
            return Json(ApiResponse::error(&format!(
                "Failed to create mint_to: {}",
                e
            )))
        }
    };

    let serialized = bincode::serialize(&ix).unwrap();
    let data_b64 = general_purpose::STANDARD.encode(serialized);
    Json(ApiResponse::ok(data_b64))
}
