use crate::api_response::ApiResponse;
use axum::Json;
use base64::{engine::general_purpose, Engine};
use bincode;
use serde::Deserialize;
use solana_sdk::{pubkey::Pubkey, system_instruction};
use spl_token::instruction::transfer as spl_transfer;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct SolTransferRequest {
    pub from_pubkey: String,
    pub to_pubkey: String,
    pub lamports: u64,
}

pub async fn transfer_sol(Json(req): Json<SolTransferRequest>) -> Json<ApiResponse<String>> {
    let from = match Pubkey::from_str(&req.from_pubkey) {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid from_pubkey")),
    };

    let to = match Pubkey::from_str(&req.to_pubkey) {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid to_pubkey")),
    };

    let ix = system_instruction::transfer(&from, &to, req.lamports);
    let serialized = bincode::serialize(&ix).unwrap();
    let data_b64 = general_purpose::STANDARD.encode(serialized);
    Json(ApiResponse::ok(data_b64))
}

#[derive(Deserialize)]
pub struct TokenTransferRequest {
    pub source: String,
    pub destination: String,
    pub owner: String,
    pub amount: u64,
}

pub async fn transfer_token(Json(req): Json<TokenTransferRequest>) -> Json<ApiResponse<String>> {
    let source = match Pubkey::from_str(&req.source) {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid source pubkey")),
    };

    let dest = match Pubkey::from_str(&req.destination) {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid destination pubkey")),
    };

    let owner = match Pubkey::from_str(&req.owner) {
        Ok(pk) => pk,
        Err(_) => return Json(ApiResponse::error("Invalid owner pubkey")),
    };

    let ix = match spl_transfer(&spl_token::id(), &source, &dest, &owner, &[], req.amount) {
        Ok(instruction) => instruction,
        Err(e) => {
            return Json(ApiResponse::error(&format!(
                "Failed to create transfer: {}",
                e
            )))
        }
    };

    let serialized = bincode::serialize(&ix).unwrap();
    let data_b64 = general_purpose::STANDARD.encode(serialized);
    Json(ApiResponse::ok(data_b64))
}
