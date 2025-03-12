use dotenv::dotenv;
use web3::types::{Address, BlockId, BlockNumber};
use futures::StreamExt;

#[tokio::main]
async fn main() -> web3::Result<()> {
    dotenv().ok();

    // Connect to Ethereum via WebSockets for realtime

    // let websocket = web3::transports::WebSocket::new("wss://mainnet.infura.io/ws/v3/62aec48f670f42aa88de8fcd3ae71a0e").await?;
    let websocket = web3::transports::WebSocket::new("wss://sepolia.infura.io/ws/v3/dccf0f92e7d3450fb9e5eb8f49bd84f5").await?;

    
    let web3 = web3::Web3::new(websocket);

    // Target Eth address to fetch

    // let target_address: Address = "0x3328F7f4A1D1C57c35df56bBf0c9dCAFCA309C49".parse().unwrap();
    let target_address: Address = "0x40C34974068CBe7Ef930e4585a68740a7aee2B89".parse().unwrap();

    println!("Listening for transactions to: {:?}", target_address);

    // Subscribe to new blocks
    let mut sub = web3.eth_subscribe().subscribe_new_heads().await?;

    while let Some(block) = sub.next().await 
    {
        let block = block?; // Unwrap Result
        println!("Block Number: {:?}", block.number.unwrap());

        // Get full block details
        if let Some(block_number) = block.number 
        {
            let block_data = web3.eth().block_with_txs(BlockId::Number(BlockNumber::Number(block_number))).await?;

            if let Some(block) = block_data {
                for tx in block.transactions {
                    if Some(target_address) == tx.to {
                            let eth_value = tx.value.as_u128() as f64 / 1e18;// convert Gwei to ETH
                            println!("Incoming Tx: {}\n   From: {:?}\n   Amount: {} ETH\n", tx.hash, tx.from, eth_value);
                    }
                }
            }
        }
    }

    Ok(())
}
