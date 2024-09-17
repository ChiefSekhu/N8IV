use std::collections::HashMap;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use quantum_sig::QuantumSig;  // Using the futuristic quantum-resistant signature scheme

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub previous_hashes: Vec<String>,
    pub data: String,
    pub signature: String,  // Each block contains a quantum-resistant signature
}

#[derive(Clone)]
pub struct N8IVChain {
    pub blocks: HashMap<String, Block>,
    pub genesis_block: Block,
}

impl N8IVChain {
    pub fn new() -> Self {
        let genesis_block = Block {
            hash: String::from("genesis"),
            previous_hashes: vec![],
            data: String::from("Genesis Block"),
            signature: QuantumSig::generate_signature("Genesis Block").unwrap(),
        };
        let mut blocks = HashMap::new();
        blocks.insert(genesis_block.hash.clone(), genesis_block.clone());
        Self {
            blocks,
            genesis_block,
        }
    }

    pub async fn start(&mut self) {
        println!("N8IV Blockchain is up and running.");
    }

    pub fn add_block(&mut self, previous_hashes: Vec<String>, data: String) {
        let hash = self.generate_hash(&data);
        let signature = QuantumSig::generate_signature(&data).unwrap();
        let new_block = Block {
            hash,
            previous_hashes,
            data,
            signature,
        };
        self.blocks.insert(new_block.hash.clone(), new_block);
    }

    fn generate_hash(&self, data: &str) -> String {
        format!("{:x}", Sha256::digest(data.as_bytes()))
    }
}
