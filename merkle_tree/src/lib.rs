use sha2::{Sha256, Sha512, Digest};
use blake2::{Blake2b512, digest::Update};
use hex;
use std::marker::PhantomData;

// define hasher
pub trait Hasher {
    fn hash(data: &[u8]) -> String;
}

// implement hasher for sha256
pub struct Sha256Hasher;
impl Hasher for Sha256Hasher {
    fn hash(data: &[u8]) -> String {
        hex::encode(Sha256::digest(data))
    }
}

// implement hasher for blake2
pub struct Blake2bHasher;
impl Hasher for Blake2bHasher {
    fn hash(data: &[u8]) -> String {
        let mut hasher = Blake2b512::new();
        digest::Update::update(&mut hasher, data); 
        hex::encode(hasher.finalize())
    }
}

// implement hasher for sha512
pub struct Sha512Hasher;
impl Hasher for Sha512Hasher {
    fn hash(data: &[u8]) -> String {
        hex::encode(Sha512::digest(data))
    }
}

#[derive(Debug, Clone)]
pub struct MerkleTree<H: Hasher> {
    pub leaves: Vec<Vec<u8>>,
    pub root: String,
    _hasher: PhantomData<H>,
}

impl<H: Hasher> MerkleTree<H> {
    
    // create a new markletree from given leaves 
    pub fn new(leaves: Vec<Vec<u8>>) -> Self {
        //make root form given leaves
        let  root = Self::build_tree(&leaves);
        // root.push('h');
        //return tree
        MerkleTree { leaves, root, _hasher: PhantomData }
    }

    // build tree and return root
    fn build_tree(leaves: &Vec<Vec<u8>>) -> String  {
        
        //make hash of each leaf
        let mut hashes: Vec<String> = leaves.iter().map(|leaf| H::hash(leaf)).collect();

        
        // print all leaves hash 
        for (i, hash) in hashes.iter().enumerate() {
            println!("Leaf {} : {:?}", i+1, hash);
        }
        println!("");
        let mut level = 0;
        println!("Level {} (Leaves): {:?}\n", level, hashes);
        
        //combaining current lavel's
        while hashes.len() > 1 {
            level += 1;
            let mut new_hashes = vec![];
            let mut level_structure = vec![];

            for chunk in hashes.chunks(2) {
                let combined_hash = if chunk.len() == 2 {
                    let (left, right) = if chunk[0] < chunk[1] {
                        (chunk[0].clone(), chunk[1].clone())
                    } else {
                        (chunk[1].clone(), chunk[0].clone())
                    };
                    level_structure.push((left.clone(), right.clone()));
                    //concatinace hashes
                    H::hash(format!("{}{}", left, right).as_bytes())
                } else {
                    // level_structure.push((chunk[0].clone(), chunk[0].clone()));
                    level_structure.push((chunk[0].clone(), "carry forword".to_string()));
                    // if odd then carry forword 
                    chunk[0].clone() 
                };
                
                new_hashes.push(combined_hash);
            }
            println!("Level {}: {:?}\n", level, level_structure);
            hashes = new_hashes;
        }
        

        // println!("Merkle Root: {:?}", hashes[0]);
        //signle reaming is root hash
        hashes[0].clone()
    }

    //return root
    pub fn get_root(&self) -> String  {
        self.root.clone()
    }

    
    //proof of vector of tuple 
    // boolean indicate that right(True) and left(false)
    pub fn get_proof(&self, index: usize) -> Vec<(String, String)> {
        let mut proof: Vec<(String, String)> = vec![];
        //start with bottom lavel hashes 
        let mut hashes: Vec<String> = self.leaves.iter().map(|leaf| H::hash(leaf)).collect();
        //for current index current lavel
        let mut idx = index;

        //for check all nodes 
        while hashes.len() > 1 {
            let mut new_hashes = vec![];
            
            //process pair hashes 
            for i in (0..hashes.len()).step_by(2) {
                if i + 1 < hashes.len() {
                    // place small left and big right
                    let (left, right) = if hashes[i] < hashes[i + 1] {
                        (hashes[i].clone(), hashes[i + 1].clone())
                    } else {
                        (hashes[i + 1].clone(), hashes[i].clone())
                    };
                    //compute parent hash
                    let combined_hash = H::hash(format!("{}{}", left, right).as_bytes());

                    // println!("combined_hash: {:?}", combined_hash);

                    if i == idx || i + 1 == idx {
                        proof.push((left.clone(), right.clone()));
                    }
                    new_hashes.push(combined_hash);
                } else {
                    new_hashes.push(hashes[i].clone());
                }
            }
            hashes = new_hashes;
            //update index 
            idx /= 2;
        }

        proof
    }

    //verfies a proof against a given root 
    pub fn verify_proof(leaf: Vec<u8>,proof: Vec<(String, String)>, root: String) -> bool {
        let mut hash = H::hash(&leaf);

        for (left, right) in proof {
            let (small, big) = if left < right { (left, right) } else { (right, left) };
            hash = H::hash(format!("{}{}", small, big).as_bytes());
        }

        // validate true if final computed hash matches the root
        print!("Hash: {:?}",hash);
        hash == root
    }


}



