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

    // if balance_sender < value + (gas_limit * gas_price) 
    // {
    //     println!("Insufficient balance for transaction.");
    //     return Ok(());
    // }

    // let nonce = web3.eth().transaction_count(sender, None).await?;
    let chain_id = web3.eth().chain_id().await?.as_u64();

        //intreracting with contract
        println!("Interacting with the smart contract...");

        let contract_address: Address = env::var("CONTRACT_ADDRESS").expect("CONTRACT_ADDRESS not found in .env").parse::<H160>().unwrap();
        let contract = Contract::from_json(web3.eth(), contract_address, include_str!("storage_abi.json").as_bytes()).unwrap();

        println!("Past Event Values");
        let past_filter = FilterBuilder::default()
        .address(vec![contract_address])
        .topics(Some(vec![H256::from_slice(ethers::utils::keccak256("NumberUpdatedEvent(address)").as_ref())]), None, None, None)
        .from_block(web3::types::BlockNumber::Earliest)
        .to_block(web3::types::BlockNumber::Latest)
        .build();

        let past_events: Vec<web3::types::Log> = web3.eth().logs(past_filter).await.unwrap();

        println!("Fetching past events...");
        for log_entry in past_events {
            let raw_log = RawLog {
                topics: log_entry.topics,
                data: log_entry.data.0,
            };

            if let Ok(decoded_log) = contract.abi().event("NumberUpdatedEvent").unwrap().parse_log(raw_log) {
                let sender: Address = decoded_log.params[0].value.clone().into_address().unwrap();
                let tx_hash: H256 = log_entry.transaction_hash.unwrap();
                let block_number = log_entry.block_number.unwrap();

                // Retrieve value stored at that time
                let stored_value: U256 = contract.query("retrieve", (), sender, Options::default(), Some(block_number.into())).await.unwrap();

                println!("========================================");
                println!("Past events and that values");
                println!("Transaction Hash: {:?}", tx_hash);
                println!("Sender: {:?}", sender);
                println!("Block Number: {:?}", block_number);
                println!("Stored Value at that Time: {}", stored_value);
            }
        }

    Ok(())
}


