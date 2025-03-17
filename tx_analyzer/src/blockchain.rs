use ethers::providers::{Http, Provider, Middleware};
use ethers::types::{Address, Transaction as EthTransaction, U256};
use petgraph::graph::{DiGraph, NodeIndex};
use std::sync::Arc;

const SEPOLIA_RPC: &str = "https://sepolia.infura.io/v3/dccf0f92e7d3450fb9e5eb8f49bd84f5"; // Replace with your Infura Key

#[derive(Clone, Debug)]
pub struct Transaction {
    pub from: Address,
    pub to: Address,
    pub value: U256,
    pub hash: String,
}

pub async fn fetch_transactions(address: &str, limit: usize) -> Result<(Vec<Transaction>, DiGraph<String, String>), Box<dyn std::error::Error>> {

    let provider = Provider::<Http>::try_from(SEPOLIA_RPC)?;
    let provider = Arc::new(provider);
    
    let address: Address = address.parse()?;
    let mut transactions = Vec::new();
    let mut graph = DiGraph::<String, String>::new();

    let latest_block = provider.get_block_number().await?;
    println!("Fetching transactions...");

    for i in (0..limit).rev() {
        println!("Fetching block {}", latest_block - i as u64);
        if let Some(block) = provider.get_block_with_txs(latest_block - i as u64).await? {
            for tx in block.transactions {
                if tx.from == address || tx.to == Some(address) {
                    let from = tx.from.to_string();
                    let to = tx.to.unwrap_or_default().to_string();
                    let hash = tx.hash.to_string();

                    transactions.push(Transaction {
                        from: tx.from,
                        to: tx.to.unwrap_or_default(),
                        value: tx.value,
                        hash: tx.hash.to_string(),
                    });

                    let from_idx = graph.node_indices().find(|&i| graph[i] == from).unwrap_or_else(|| graph.add_node(from.clone()));
                    let to_idx = graph.node_indices().find(|&i| graph[i] == to).unwrap_or_else(|| graph.add_node(to.clone()));

                    graph.add_edge(from_idx, to_idx, hash);
                }
            }
        }
    }

    println!("Transaction graph created with {} nodes.", graph.node_count());
    Ok((transactions, graph))
}


