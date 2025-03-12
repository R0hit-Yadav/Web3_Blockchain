use axum::{
    routing::{get, post},
    Json, Router,
};
use ethers::prelude::*;
use ethers::signers::LocalWallet;
use serde::{Deserialize, Serialize};
use std::env;
use std::sync::Arc;
use tokio::sync::Mutex;
use tower_http::cors::{Any, CorsLayer};
use web3::types::{TransactionReceipt, U256};
use web3::{transports::Http, Web3};
use dotenv::dotenv;

// Structs for API requests
#[derive(Deserialize)]
struct TransactionRequest {
    sender_private_key: String,
    recipient: String,
    amount: f64,
}

#[derive(Serialize)]
struct TransactionResponse {
    tx_hash: String,
}


// Ethereum client state
struct AppState {
    web3: Web3<Http>,
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    let eth_rpc_url = env::var("ETHEREUM_RPC_URL").expect("Missing ETHEREUM_RPC_URL");

    let state = Arc::new(Mutex::new(AppState {
        web3: Web3::new(Http::new(&eth_rpc_url).unwrap()),
    }));

    let app = Router::new()
        .route("/", get(health_check))
        .route("/send_transaction", post(send_transaction))
        .layer(CorsLayer::new().allow_origin(Any)); // Allow frontend requests

    println!("🚀 Server running on http://localhost:8000");
    axum::Server::bind(&"0.0.0.0:8000".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn health_check() -> &'static str {
    "Backend is running"
}

async fn send_transaction(
    Json(payload): Json<TransactionRequest>,
) -> Json<TransactionResponse> {
    let sender_wallet: LocalWallet = payload.sender_private_key.parse().unwrap();
    let sender = sender_wallet.address();

    let web3 = Web3::new(Http::new(&env::var("ETHEREUM_RPC_URL").unwrap()).unwrap());
    let nonce = web3.eth().transaction_count(sender, None).await.unwrap();
    let chain_id = web3.eth().chain_id().await.unwrap().as_u64();
    let gas_price = web3.eth().gas_price().await.unwrap();
    let gas_limit = U256::from(21000);
    let value = ethers::utils::parse_units(payload.amount, "ether").unwrap();

    let mut tx: TypedTransaction = TransactionRequest {
        from: Some(sender),
        to: Some(payload.recipient.parse().unwrap()),
        value: Some(value),
        gas: Some(gas_limit),
        gas_price: Some(gas_price),
        nonce: Some(nonce),
        ..Default::default()
    }
    .into();

    tx.set_chain_id(chain_id);
    let signature = sender_wallet.sign_transaction(&tx).await.unwrap();
    let bytes: Vec<u8> = tx.rlp_signed(&signature).to_vec();
    let tx_hash = web3.eth().send_raw_transaction(web3::types::Bytes(bytes)).await.unwrap();

    Json(TransactionResponse {
        tx_hash: format!("{:?}", tx_hash),
    })
}
