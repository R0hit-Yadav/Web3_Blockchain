use ethers::types::TransactionRequest as EthersTxRequest;
use ethers::types::transaction::eip2718::TypedTransaction;
use web3::contract::{Contract, Options};
use ethers::abi::{Abi, Address, RawLog, Token};
use ethers::prelude::*;
use ethers::signers::LocalWallet;
use web3::transports::WebSocket;
use web3::types::{FilterBuilder, H160, H256, U256};
use web3::Web3;
use dotenv::dotenv;
use std::env;


#[tokio::main]
pub async fn main() -> web3::Result<()> {

    dotenv().ok(); // Load .env file


    // Connect to Ethereum node (Infura, Alchemy, or Local Node)
    let transport = WebSocket::new(&env::var("ETHEREUM_WS_URL").expect("Missing ETHEREUM_WS_URL")).await?;
    let web3 = Web3::new(transport);


    // Send transaction (if balance is sufficient)
    let sender_wallet: LocalWallet = env::var("SENDER_PRIVATE_KEY").expect("Missing SENDER_PRIVATE_KEY").parse().unwrap();
    let sender = sender_wallet.address();

    //check balance of Sender
    let balance_sender: U256 = web3.eth().balance(sender, None).await?;
    let balance_in_eth_sender = ethers::utils::format_units(balance_sender, "ether").unwrap();
    println!("Balance Of Sender: {} ETH", balance_in_eth_sender);

    let (gas_price, gas_limit, value) = (web3.eth().gas_price().await?, U256::from(21000), U256::exp10(16));

    if balance_sender < value + (gas_limit * gas_price) 
    {
        println!("Insufficient balance for transaction.");
        return Ok(());
    }

    // let nonce = web3.eth().transaction_count(sender, None).await?;
    let chain_id = web3.eth().chain_id().await?.as_u64();

        //intreracting with contract
        println!("Interacting with the smart contract...");

        let contract_address: Address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not found in .env").parse::<H160>().unwrap();
        let contract = Contract::from_json(web3.eth(), contract_address, include_str!("storage_abi.json").as_bytes()).unwrap();




        // Filter for the `NumberUpdatedEvent`
        let filter = FilterBuilder::default().address(vec![contract_address]).topics
        (Some(vec![H256::from_slice(ethers::utils::keccak256("NumberUpdatedEvent(address)").as_ref()),]),None,None,None,)
        .build();
    
        let mut event_stream = web3.eth_subscribe().subscribe_logs(filter).await.unwrap();
            

        // Store a new value in the contract
        let store_value: U256 = 15.into();
        let nonce = web3.eth().transaction_count(sender, None).await?;

        let mut tx: TypedTransaction = EthersTxRequest 
        {
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
        // println!("Stored Value Transaction Hash: {:?}", tx_hash);


        println!("Waiting for transaction Confirmation...");
        while web3.eth().transaction_receipt(tx_hash).await?.is_none() {
            tokio::time::sleep(std::time::Duration::from_millis(500)).await;
        }
        println!("Transaction confirmed!");
        println!("");


        println!("Listening for NumberUpdatedEvent...");
        while let Some(log) = event_stream.next().await 
        {
            match log {
                Ok(log_entry) => {
                    let raw_log = RawLog {
                        topics: log_entry.topics,
                        data: log_entry.data.0,
                    };
    
                    if let Ok(decoded_log) = contract.abi().event("NumberUpdatedEvent").unwrap().parse_log(raw_log) {
                        let log_index = log_entry.log_index.unwrap();
                        let sender: Address = decoded_log.params[0].value.clone().into_address().unwrap();
                        let tx_hash: H256 = log_entry.transaction_hash.unwrap();
                        let tx_index = log_entry.transaction_index.unwrap();
                        let block_number = log_entry.block_number.unwrap();
                        let block_hash: H256 = log_entry.block_hash.unwrap();
    
                        println!("Event Received!");
                        println!("Log Index: {}", log_index);
                        println!("Transaction Hash: {:?}", tx_hash);
                        println!("Transaction Index: {:?}", tx_index);
                        println!("Sender: {:?}", sender);
                        println!("Block Number: {:?}", block_number);
                        println!("Block Hash: {:?}", block_hash);
    
                        // Retrieve the updated value
                        let stored_value: U256 = contract.query("retrieve", (), None, Options::default(), None).await.unwrap();
                        println!("Event Value in Contract: {}", stored_value);
                    }
                }
                Err(e) => println!("Error listening to event: {:?}", e),
            }
            break;
        }



        // Retrieve stored value
        let stored_value: U256 = contract.query("retrieve", (), None, Options::default(), None).await.unwrap();
        println!("Stored Value in Contract: {}", stored_value);

    Ok(())
}


