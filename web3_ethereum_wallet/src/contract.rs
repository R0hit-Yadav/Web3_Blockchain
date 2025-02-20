use ethers::prelude::*;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::types::TransactionRequest as EthersTxRequest;
use web3::types::Bytes;
use ethers::signers::LocalWallet;
use web3::{transports::Http, types::U256, Web3};
use dotenv::dotenv;
use std::env;
use hex::encode;
use web3::contract::{Contract, Options};
use ethers::abi::Token;
use tokio::time::sleep;
use std::time::Duration;

#[tokio::main]
pub async fn main() -> web3::Result<()> {
    dotenv().ok();

    let wallet = LocalWallet::new(&mut rand::thread_rng());
    let (wallet_address, private_key) = (wallet.address(), encode(wallet.signer().to_bytes()));

    println!("New Ethereum Wallet Address: {:?}", wallet_address);
    println!("Private Key: {:?}", private_key);

    let web3 = Web3::new(Http::new(&env::var("ETHEREUM_RPC_URL").expect("Missing ETHEREUM_RPC_URL"))?);
    
    let sender_wallet: LocalWallet = env::var("SENDER_PRIVATE_KEY").expect("Missing SENDER_PRIVATE_KEY").parse().unwrap();
    let sender = sender_wallet.address();
    
    let balance_sender: U256 = web3.eth().balance(sender, None).await?;
    let balance_in_eth_sender = ethers::utils::format_units(balance_sender, "ether").unwrap();
    println!("Balance Of Sender: {} ETH", balance_in_eth_sender);

    let (gas_price, gas_limit, value) = (web3.eth().gas_price().await?, U256::from(21000), U256::exp10(16));
    
    if balance_sender < value + (gas_limit * gas_price) {
        println!("Insufficient balance for transaction.");
        return Ok(());
    }

    let chain_id = web3.eth().chain_id().await?.as_u64();
    let nonce = web3.eth().transaction_count(sender, None).await?;
    
    let tx: TypedTransaction = EthersTxRequest {
        from: Some(sender),
        to: Some(NameOrAddress::Address(wallet_address)),
        gas: Some(gas_limit),
        gas_price: Some(gas_price),
        nonce: Some(nonce),
        value: Some(value),
        chain_id: Some(chain_id.into()),
        ..Default::default()
    }
    .into();

    let signature = sender_wallet.sign_transaction(&tx).await.unwrap();
    let bytes: Vec<u8> = tx.rlp_signed(&signature).to_vec();
    let tx_hash = web3.eth().send_raw_transaction(Bytes::from(bytes)).await.unwrap();
    println!("Transaction sent! Hash: {:?}", tx_hash);

    while web3.eth().transaction_receipt(tx_hash).await?.is_none() {
        sleep(Duration::from_millis(10)).await;
    }

    let receipt = web3.eth().transaction_receipt(tx_hash).await?.unwrap();
    println!(
        "=============================\n\
         ||==>Transaction Receipt:<===\n\
         =============================\n\
         ||  Transaction Index: {:?}\n\
         ||  Transaction Hash: {:?}\n\
         ||  Block Number: {:?}\n\
         ||  From: {:?}\n\
         ||  To: {:?}\n\
         ||  Gas Used: {:?}\n\
         ||  Status: {}\n\
         =============================",
        receipt.transaction_index,
        receipt.transaction_hash,
        receipt.block_number.unwrap_or_default(),
        receipt.from,
        receipt.to.unwrap_or_default(),
        receipt.gas_used.unwrap_or_default(),
        if receipt.status == Some(1.into()) { "Success" } else { "Failed" }
    );

    println!("Interacting with the smart contract...");

    let contract_address: H160 = env::var("CONTRACT_ADDRESS").expect("Missing CONTRACT_ADDRESS").parse().unwrap();
    let contract = Contract::from_json(web3.eth(), contract_address, include_str!("storage_abi.json").as_bytes()).unwrap();

    let store_value: U256 = 54.into();
    let nonce = web3.eth().transaction_count(sender, None).await?;

    let tx: TypedTransaction = EthersTxRequest {
        from: Some(sender),
        to: Some(NameOrAddress::Address(contract_address)),
        gas: Some(U256::from(100000)),
        gas_price: Some(gas_price),
        nonce: Some(nonce),
        value: Some(U256::zero()), // No ETH transfer, just function call
        data: Some(contract.abi().function("store").unwrap().encode_input(&[Token::Uint(store_value)]).unwrap().into()),
        chain_id: Some(chain_id.into()),
        ..Default::default()
    }
    .into();

    let signature = sender_wallet.sign_transaction(&tx).await.unwrap();
    let bytes: Vec<u8> = tx.rlp_signed(&signature).to_vec();
    let tx_hash = web3.eth().send_raw_transaction(Bytes::from(bytes)).await.unwrap();
    println!("Stored Value Transaction Hash: {:?}", tx_hash);

    while web3.eth().transaction_receipt(tx_hash).await?.is_none() {
        sleep(Duration::from_secs(1)).await;
    }
    
    println!("Transaction confirmed!");
    let stored_value: U256 = contract.query("retrieve", (), None, Options::default(), None).await.unwrap();
    println!("Stored Value in Contract: {}", stored_value);

    Ok(())
}
