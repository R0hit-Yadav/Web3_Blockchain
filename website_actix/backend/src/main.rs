use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use actix_cors::Cors;
use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::TransactionRequest as EthersTxRequest;
use web3::transports::Http;
use web3::types::U256;
use web3::Web3;
use dotenv::dotenv;
use hex::encode;
use serde::{Deserialize, Serialize};
use std::env;

#[derive(Serialize)]
struct WalletResponse {
    address: String,
    private_key: String,
    balance: String,
}

#[derive(Serialize)]
struct BalanceResponse {
    address: String,
    balance: String,
}

#[derive(Serialize)]
struct TransactionResponse {
    tx_hash: String,
    receipt: Option<TransactionReceipt>,
}

#[derive(Serialize)]
struct TransactionReceipt {
    transaction_index: u64,
    transaction_hash: String,
    block_number: u64,
    from: String,
    to: String,
    gas_used: String,
    status: String,
}

#[derive(Deserialize)]
struct BalanceQuery {
    address: String,
}

#[derive(Deserialize)]
struct TransactionRequest {
    sender_private_key: String,
    receiver_address: String,
}

async fn generate_wallet() -> impl Responder {
    dotenv().ok();
    let wallet = LocalWallet::new(&mut rand::thread_rng());
    let wallet_address = format!("{:?}", wallet.address());
    let private_key = encode(wallet.signer().to_bytes());
    let web3 = Web3::new(Http::new(&env::var("ETHEREUM_RPC_URL").expect("Missing ETHEREUM_RPC_URL")).unwrap());
    let balance: U256 = web3.eth().balance(wallet.address(), None).await.unwrap();
    let balance_in_eth = ethers::utils::format_units(balance, "ether").unwrap();

    HttpResponse::Ok().json(WalletResponse {
        address: wallet_address,
        private_key,
        balance: balance_in_eth,
    })
}

async fn get_balance(query: web::Query<BalanceQuery>) -> impl Responder {
    dotenv().ok();
    let web3 = Web3::new(Http::new(&env::var("ETHEREUM_RPC_URL").expect("Missing ETHEREUM_RPC_URL")).unwrap());
    let address: Address = match query.address.parse() {
        Ok(addr) => addr,
        Err(_) => return HttpResponse::BadRequest().body("Invalid Ethereum address"),
    };
    let balance: U256 = match web3.eth().balance(address, None).await {
        Ok(b) => b,
        Err(_) => return HttpResponse::InternalServerError().body("Failed to fetch balance"),
    };
    let balance_in_eth = ethers::utils::format_units(balance, "ether").unwrap();

    HttpResponse::Ok().json(BalanceResponse {
        address: query.address.clone(),
        balance: balance_in_eth,
    })
}

async fn send_transaction(data: web::Json<TransactionRequest>) -> impl Responder {
    println!("Received POST /send-transaction");
    dotenv().ok();
    let web3 = Web3::new(Http::new(&env::var("ETHEREUM_RPC_URL").expect("Missing ETHEREUM_RPC_URL")).unwrap());
    let sender_wallet: LocalWallet = match data.sender_private_key.parse() {
        Ok(wallet) => wallet,

        Err(_) => return HttpResponse::BadRequest().body("Invalid sender private key"),
    };
    let sender = sender_wallet.address();
    let recv_addr: Address = match data.receiver_address.parse() {
        Ok(addr) => addr,
        Err(_) => return HttpResponse::BadRequest().body("Invalid receiver address"),
    };

    let balance_sender: U256 = web3.eth().balance(sender, None).await.unwrap();
    let (gas_price, gas_limit, value) = (web3.eth().gas_price().await.unwrap(), U256::from(21000), U256::exp10(16));

    if balance_sender < value + (gas_limit * gas_price) {
        return HttpResponse::BadRequest().body("Insufficient balance for transaction.");
    }

    let nonce = web3.eth().transaction_count(sender, None).await.unwrap();
    let chain_id = web3.eth().chain_id().await.unwrap().as_u64();

    let mut tx: TypedTransaction = EthersTxRequest {
        from: Some(sender),
        to: Some(recv_addr.into()),
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

    while web3.eth().transaction_receipt(tx_hash).await.unwrap().is_none() {
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }

    let receipt = web3.eth().transaction_receipt(tx_hash).await.unwrap().unwrap();

    let receipt_data = TransactionReceipt {
        transaction_index: receipt.transaction_index.as_u64(),
        transaction_hash: format!("{:?}", receipt.transaction_hash),
        block_number: receipt.block_number.unwrap_or_default().as_u64(),
        from: format!("{:?}", receipt.from),
        to: format!("{:?}", receipt.to.unwrap_or_default()),
        gas_used: format!("{:?}", receipt.gas_used.unwrap_or_default()),
        status: if receipt.status == Some(1.into()) { "Success" } else { "Failed" }.to_string(),
    };

    HttpResponse::Ok().json(TransactionResponse {
        tx_hash: format!("{:?}", tx_hash),
        receipt: Some(receipt_data),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Server running on http://127.0.0.1:8080");
    HttpServer::new(|| {
      let app = App::new()
            .wrap(
                Cors::default()
                    .allowed_origin("http://localhost:5173")
                    .allowed_methods(vec!["GET", "POST"])
                    .allowed_headers(vec![actix_web::http::header::CONTENT_TYPE])
                    .supports_credentials()
                    .max_age(3600),
            )
            .route("/generate-wallet", web::get().to(generate_wallet))
            .route("/get-balance", web::get().to(get_balance));
        println!("Registered routes: /generate-wallet, /get-balance, /send-transaction");
        app.route("/send-transaction", web::post().to(send_transaction))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}