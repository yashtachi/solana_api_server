mod api_response;
mod endpoints;

use axum::{routing::post, Router};
use endpoints::{
    keypair::generate_keypair,
    message::{sign_message, verify_message},
    token::{init_mint, mint_tokens},
    transfer::{transfer_sol, transfer_token},
};
use std::net::SocketAddr;
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize tracing for better logging
    tracing_subscriber::fmt::init();

    let app = Router::new()
        .route(
            "/",
            axum::routing::get(|| async { "Solana API Server is running!" }),
        )
        .route("/health", axum::routing::get(|| async { "OK" }))
        .route("/keypair", post(generate_keypair))
        .route("/token/create", post(init_mint))
        .route("/token/mint", post(mint_tokens))
        .route("/message/sign", post(sign_message))
        .route("/message/verify", post(verify_message))
        .route("/send/sol", post(transfer_sol))
        .route("/send/token", post(transfer_token));

    // Railway provides PORT environment variable
    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "3000".to_string())
        .parse::<u16>()
        .map_err(|e| format!("Invalid PORT value: {}", e))?;

    // Bind to 0.0.0.0 for Railway deployment
    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    println!("🚀 Solana API Server starting on {}", addr);

    let listener = TcpListener::bind(addr)
        .await
        .map_err(|e| format!("Failed to bind to {}: {}", addr, e))?;

    println!("✅ Server successfully bound to {}", addr);

    axum::serve(listener, app)
        .await
        .map_err(|e| format!("Server error: {}", e))?;

    Ok(())
}
