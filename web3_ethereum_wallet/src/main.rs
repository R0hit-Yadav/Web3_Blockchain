use ethers::types::TransactionRequest as EthersTxRequest;
use ethers::types::transaction::eip2718::TypedTransaction;
use ethers::core::rand::thread_rng;
use ethers::prelude::*;
use ethers::signers::LocalWallet;
use web3::transports::Http;
use web3::types::U256;
use web3::Web3;
use dotenv::dotenv;
use std::env;
use hex::encode;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv().ok(); // Load .env file

    // Generate a new Ethereum wallet
    let wallet = LocalWallet::new(&mut thread_rng());
    let wallet_address = wallet.address();
    let private_key = encode(wallet.signer().to_bytes()); 

    //receiver address
    // let recv_sddr= "0xf131Dd488dAC83a7fb5A8bB9f57d05a1e54ef100"; // chnage to specific 


    println!("New Ethereum Wallet Address: {:?}", wallet_address);
    println!("Private Key: {:?}", private_key);

    // Connect to Ethereum node (Infura, Alchemy, or Local Node)
    let rpc_url = env::var("ETHEREUM_RPC_URL").expect(" ETHEREUM_RPC_URL not found in .env");
    let transport = Http::new(&rpc_url).unwrap(); //unwrap to solve error
    let web3 = Web3::new(transport);


    // Fetch balance
    let balance: U256 = web3.eth().balance(wallet_address, None).await?;
    let balance_in_eth = ethers::utils::format_units(balance, "ether").unwrap();
    println!("Balance New Wallet: {} ETH", balance_in_eth);

    // Send transaction (if balance is sufficient)
    let sender_private_key = env::var("SENDER_PRIVATE_KEY").expect("SENDER_PRIVATE_KEY not found in .env");
    let sender_wallet: LocalWallet = sender_private_key.parse().unwrap();
    let sender = sender_wallet.address();

    let value = U256::exp10(16); // 16=0.01 ETH //18=1 ETH
    let gas_limit = U256::from(21000);

    //check balance of Sender
    let balance_sender: U256 = web3.eth().balance(sender, None).await?;
    let balance_in_eth_sender = ethers::utils::format_units(balance_sender, "ether").unwrap();
    println!("Balance Of Sender: {} ETH", balance_in_eth_sender);


    let gas_price = web3.eth().gas_price().await?; // get current gas price
    let total_gas_cost = gas_limit * gas_price; // total gas cost (gas_limit * gas_price)

    if balance_sender < value + total_gas_cost {
        println!("Insufficient balance for transaction.");
        return Ok(());
   }

    let nonce = web3.eth().transaction_count(sender, None).await?;
    let chain_id = web3.eth().chain_id().await?.as_u64();
    // println!("Chain ID: {:?}", chain_id);

    let mut tx: TypedTransaction = EthersTxRequest {
        from: Some(sender),
        to: Some(ethers::types::NameOrAddress::Address(wallet_address)),
        // to:Some(recv_sddr.parse().unwrap()),// uncomment this line to send to specific address
        value: Some(value),
        gas: Some(gas_limit),
        gas_price: Some(gas_price),
        nonce: Some(nonce),
        ..Default::default()
    }
    .into();

    tx.set_chain_id(chain_id);

    let signature = sender_wallet.sign_transaction(&tx).await.unwrap();
    let rlp_signed_tx = tx.rlp_signed(&signature);

    let tx_hash = web3.eth().send_raw_transaction(web3::types::Bytes(rlp_signed_tx.0.to_vec())).await?;
    println!("Transaction sent! Hash: {:?}", tx_hash);

    println!("Transaction Receipt Generating....");

    let mut receipt = None;
    while receipt.is_none() 
    {
        receipt = web3.eth().transaction_receipt(tx_hash).await?;
        if receipt.is_none() 
        {
            tokio::time::sleep(std::time::Duration::from_millis(10)).await; // Wait for 5 seconds before checking again
        }
    }
    let receipt = receipt.unwrap();
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

    Ok(())
    
}

