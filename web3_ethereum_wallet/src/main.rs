use ethers::types::TransactionRequest as EthersTxRequest;
use ethers::types::transaction::eip2718::TypedTransaction;
use web3::contract::{Contract, Options};
use ethers::prelude::*;
use ethers::signers::LocalWallet;
use web3::transports::Http;
use web3::types::U256;
use web3::Web3;
use dotenv::dotenv;
use std::env;
use hex::encode;
use ethers::abi::Token;


#[tokio::main]
async fn main() -> web3::Result<()> {

    // let mut i=0;
    // while i<10
    // {
       
    dotenv().ok(); // Load .env file

    // Generate a new Ethereum wallet
    let wallet = LocalWallet::new(&mut rand::thread_rng());
    let (wallet_address, private_key) = (wallet.address(), encode(wallet.signer().to_bytes()));

    //receiver address
    println!("");
    println!("Generating New Ethereum Wallet Address...");
    let recv_sddr= "0x40C34974068CBe7Ef930e4585a68740a7aee2B89"; // chnage to specific
    println!("New Ethereum Wallet Address: {:?}", wallet_address);
    println!("Private Key: {:?}", private_key);


    // Connect to Ethereum node (Infura, Alchemy, or Local Node)
    let web3 = Web3::new(Http::new(&env::var("ETHEREUM_RPC_URL").expect("Missing ETHEREUM_RPC_URL"))?);
    

    // Fetch balance
    let balance: U256 = web3.eth().balance(wallet_address, None).await?;
    let balance_in_eth = ethers::utils::format_units(balance, "ether").unwrap();
    println!("Balance New Wallet: {} ETH", balance_in_eth);
    println!("");

    // Send transaction (if balance is sufficient)
    let sender_wallet: LocalWallet = env::var("SENDER_PRIVATE_KEY").expect("Missing SENDER_PRIVATE_KEY").parse().unwrap();
    let sender = sender_wallet.address();

    //check balance of Sender
    let balance_sender: U256 = web3.eth().balance(sender, None).await?;
    let balance_in_eth_sender = ethers::utils::format_units(balance_sender, "ether").unwrap();
    println!("Balance Of Sender: {} ETH", balance_in_eth_sender);

    let (gas_price, gas_limit, value) = (web3.eth().gas_price().await?, U256::from(21000), U256::exp10(16));

    if balance_sender < value + (gas_limit * gas_price) {
        println!("Insufficient balance for transaction.");
        return Ok(());
    }
    println!("Sending transaction...");
    println!("");
    let nonce = web3.eth().transaction_count(sender, None).await?;
    let chain_id = web3.eth().chain_id().await?.as_u64();


    let mut tx: TypedTransaction = EthersTxRequest {
        from: Some(sender),
        // to: Some(ethers::types::NameOrAddress::Address(wallet_address)),
        to:Some(recv_sddr.parse().unwrap()),// uncomment this line to send to specific address
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
    println!("Transaction sent! \nHash: {:?}", tx_hash);

    println!("Transaction Receipt Generating....");
    println!("");

        while web3.eth().transaction_receipt(tx_hash).await?.is_none() {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }

        let receipt = web3.eth().transaction_receipt(tx_hash).await?.unwrap();
        println!("=============================");
        println!("||==>Transaction Receipt:<===");
        println!("=============================");
        println!("||  Transaction Index: {:?}", receipt.transaction_index);
        println!("||  Transaction Hash: {:?} ", receipt.transaction_hash);
        println!("||  Block Number: {:?}", receipt.block_number.unwrap_or_default());
        println!("||  From: {:?}", receipt.from);
        println!("||  To: {:?}", receipt.to.unwrap_or_default());
        println!("||  Gas Used: {:?}", receipt.gas_used.unwrap_or_default());
        println!("||  Status: {:?}", if receipt.status == Some(1.into()) { "Success" } else { "Failed" });
        println!("=============================");

        println!("");
        println!("");
        //intreracting with contract
        println!("Interacting with the smart contract...");

        let contract_address: Address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not found in .env").parse::<H160>().unwrap();
        let contract = Contract::from_json(web3.eth(), contract_address, include_str!("storage_abi.json").as_bytes()).unwrap();
        

        // Store a new value in the contract
        let store_value: U256 = 100.into();
        let nonce = web3.eth().transaction_count(sender, None).await?;


        let mut tx: TypedTransaction = EthersTxRequest {
            from: Some(sender),
            to: Some(ethers::types::NameOrAddress::Address(contract_address)),
            gas: Some(U256::from(100000)),
            gas_price: Some(gas_price),
            nonce: Some(nonce),
            data: Some(contract.abi().function("store").unwrap().encode_input(&[Token::Uint(store_value)]).unwrap().into()),
            value: Some(U256::zero()), // No ETH transfer, just function call
            ..Default::default()
        }
        .into();


        tx.set_chain_id(chain_id);
        let signature = sender_wallet.sign_transaction(&tx).await.unwrap();
        let bytes: Vec<u8> = tx.rlp_signed(&signature).to_vec();
        let tx_hash = web3.eth().send_raw_transaction(web3::types::Bytes(bytes)).await.unwrap();
        println!("Stored Value Transaction Hash: {:?}", tx_hash);
        println!("");

        println!("Waiting for transaction Confirmation...");
        while web3.eth().transaction_receipt(tx_hash).await?.is_none() {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
        println!("Transaction confirmed!");
        println!("");
        // Retrieve stored value
        let stored_value: U256 = contract.query("retrieve", (), None, Options::default(), None).await.unwrap();
        println!("Stored Value in Contract: {}", stored_value);

    //     i+=1;
    // }

    Ok(())
}


