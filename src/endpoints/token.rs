use crate::api_response::ApiResponse;
use axum::Json;
use serde::Deserialize;
use solana_sdk::pubkey::Pubkey;
use spl_token::instruction::{initialize_mint, mint_to};
// use bs58;
use base64::{engine::general_purpose, Engine};
use bincode;
use std::str::FromStr;

#[derive(Deserialize)]
pub struct CreateMint { mint_pubkey: String, mint_authority: String, decimals: u8 }

pub async fn init_mint(Json(req): Json<CreateMint>) -> Json<ApiResponse<String>> {
    let mint = Pubkey::from_str(&req.mint_pubkey).map_err(|_| "Invalid mint_pubkey");
    let auth = Pubkey::from_str(&req.mint_authority).map_err(|_| "Invalid mint_authority");
    if let (Err(e), _) | (_, Err(e)) = (&mint, &auth) {
        return Json(ApiResponse::err(e.to_string()));
    }
    let ix = initialize_mint(&spl_token::id(), &mint.unwrap(), &auth.unwrap(), None, req.decimals)
        .map_err(|e| e.to_string());
    let data_b64 = general_purpose::STANDARD.encode(bincode::serialize(&ix).unwrap());
    Json(ApiResponse::ok(data_b64))
}

#[derive(Deserialize)]
pub struct MintRequest { mint_pubkey: String, destination: String, authority: String, amount: u64 }

pub async fn mint_tokens(Json(req): Json<MintRequest>) -> Json<ApiResponse<String>> {
    let mint = Pubkey::from_str(&req.mint_pubkey).map_err(|_| "Invalid mint_pubkey");
    let dest = Pubkey::from_str(&req.destination).map_err(|_| "Invalid destination");
    let auth = Pubkey::from_str(&req.authority).map_err(|_| "Invalid authority");
    
    // Check for errors in any of the pubkeys
    if let (Err(e), _, _) | (_, Err(e), _) | (_, _, Err(e)) = (&mint, &dest, &auth) {
        return Json(ApiResponse::err(e.to_string()));
    }
    
    // All pubkeys are valid, unwrap them safely
    let ix = mint_to(
        &spl_token::id(), 
        &mint.unwrap(), 
        &dest.unwrap(), 
        &auth.unwrap(), 
        &[], 
        req.amount
    ).map_err(|e| e.to_string());
    
    match ix {
        Ok(instruction) => {
            let data_b64 = general_purpose::STANDARD.encode(bincode::serialize(&instruction).unwrap());
            Json(ApiResponse::ok(data_b64))
        },
        Err(e) => Json(ApiResponse::err(e))
    }
}