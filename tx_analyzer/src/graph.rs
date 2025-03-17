use petgraph::graph::DiGraph;
use std::collections::HashMap;
use crate::blockchain::Transaction;

pub fn build_graph(transactions: &[Transaction]) -> DiGraph<String, u128> {
    let mut graph = DiGraph::<String, u128>::new();
    let mut addresses = HashMap::new();

    println!("Building graph with {} transactions...", transactions.len());

    for tx in transactions {
        let from = tx.from.to_string();
        let to = tx.to.to_string();
        let value = tx.value.as_u128();

        let from_node = *addresses.entry(from.clone()).or_insert_with(|| graph.add_node(from));
        let to_node = *addresses.entry(to.clone()).or_insert_with(|| graph.add_node(to));

        graph.add_edge(from_node, to_node, value);
    }
    graph
}
