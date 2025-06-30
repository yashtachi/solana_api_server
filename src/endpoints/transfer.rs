use crate::api_response::ApiResponse;
use axum::Json;
use serde::Deserialize;
use solana_sdk::{pubkey::Pubkey, system_instruction};
use spl_token::instruction::transfer as spl_transfer;
use base64::{engine::general_purpose, Engine};
use bincode;
// use bs58;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct SolRequest { from_pubkey: String, to_pubkey: String, lamports: u64 }

pub async fn transfer_sol(Json(req): Json<SolRequest>) -> Json<ApiResponse<String>> {
    let from = Pubkey::from_str(&req.from_pubkey).map_err(|_| "Bad from_pubkey");
    let to = Pubkey::from_str(&req.to_pubkey).map_err(|_| "Bad to_pubkey");
    if from.is_err() || to.is_err() { return Json(ApiResponse::err("Invalid pubkey")); }
    let ix = system_instruction::transfer(&from.unwrap(), &to.unwrap(), req.lamports);
    let data_b64 = general_purpose::STANDARD.encode(bincode::serialize(&ix).unwrap());
    Json(ApiResponse::ok(data_b64))
}

#[derive(Deserialize)]
pub struct TokenTransfer { source: String, destination: String, owner: String, amount: u64 }

pub async fn transfer_token(Json(req): Json<TokenTransfer>) -> Json<ApiResponse<String>> {
    let src = Pubkey::from_str(&req.source).map_err(|_| "Bad source");
    let dst = Pubkey::from_str(&req.destination).map_err(|_| "Bad destination");
    let own = Pubkey::from_str(&req.owner).map_err(|_| "Bad owner");
    if src.is_err() || dst.is_err() || own.is_err() {
        return Json(ApiResponse::err("Invalid pubkey in transfer_token"));
    }
    let ix = spl_transfer(&spl_token::id(), &src.unwrap(), &dst.unwrap(), &own.unwrap(), &[], req.amount)
        .map_err(|e| e.to_string());
    let data_b64 = general_purpose::STANDARD.encode(bincode::serialize(&ix.unwrap()).unwrap());
    Json(ApiResponse::ok(data_b64))
}
