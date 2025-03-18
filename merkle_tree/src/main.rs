use std::io;
use merkle_tree::{MerkleTree, Sha256Hasher, Sha512Hasher,Blake2bHasher};

fn main() {
    let mut leaves = Vec::new();
    let mut input = String::new();

    println!("Enter number of leaves:");
    io::stdin().read_line(&mut input).unwrap();
    let num_leaves: usize = input.trim().parse().unwrap();
    input.clear();

    for i in 0..num_leaves {
        println!("Enter data for leaf {}:", i + 1);
        io::stdin().read_line(&mut input).unwrap();
        leaves.push(input.trim().as_bytes().to_vec());
        input.clear();
    }

    println!("Choose hash function \n1.SHA-256\n2.SHA-512\n3.Blake2b\n");
    io::stdin().read_line(&mut input).unwrap();
    let hash_choice: u32 = input.trim().parse().unwrap();
    input.clear();

    match hash_choice {
        1 => {
            let merkle_tree = MerkleTree::<Sha256Hasher>::new(leaves.clone());
            process_merkle_tree::<Sha256Hasher>(merkle_tree, &leaves);
        }
        2 => {
            let merkle_tree = MerkleTree::<Sha512Hasher>::new(leaves.clone());
            process_merkle_tree::<Sha512Hasher>(merkle_tree, &leaves);
        }
        3 => {
            let merkle_tree = MerkleTree::<Blake2bHasher>::new(leaves.clone());
            process_merkle_tree::<Blake2bHasher>(merkle_tree, &leaves);
        }
        _ => println!("Invalid choice!"),
    }

    fn process_merkle_tree<H: merkle_tree::Hasher>(merkle_tree: MerkleTree<H>, leaves: &Vec<Vec<u8>>) {
        println!("\nMerkle Root: {}", merkle_tree.get_root());
    
        for i in 0..leaves.len() {
            let proof = merkle_tree.get_proof(i);
            println!("\nProof for LEAF {}: {:?}", i + 1, proof);
    
            let is_valid = MerkleTree::<H>::verify_proof(leaves[i].clone(), proof, merkle_tree.get_root());
            println!("\nProof Verification: {}", is_valid);
        }
    }
}
