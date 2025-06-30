mod api_response;
mod endpoints;

use axum::{routing::post, Router};
use std::net::SocketAddr;
use endpoints::{keypair::generate_keypair, token::{init_mint, mint_tokens}, message::{sign_message, verify_message}, transfer::{transfer_sol, transfer_token}};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(init_mint))
        .route("/token/mint", post(mint_tokens))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(transfer_sol))
        .route("/send/token", post(transfer_token));
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on {}", addr);
    let listener = TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
