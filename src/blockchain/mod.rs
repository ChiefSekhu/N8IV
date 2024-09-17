use std::collections::HashMap;
use sha2::{Sha256, Digest};
use serde::{Serialize, Deserialize};
use ed25519_dalek::{Keypair, Signer};
use rand::rngs::OsRng;  // Ensure this import is from rand::rngs

#[derive(Clone, Serialize, Deserialize)]
pub struct Block {
    pub hash: String,
    pub previous_hashes: Vec<String>,
    pub data: String,
    pub signature: Vec<u8>,
}

pub struct N8IVChain {
    pub blocks: HashMap<String, Block>,
    pub genesis_block: Block,
    pub keypair: Keypair,  // Keypair for signing blocks
}

impl N8IVChain {
    pub fn new() -> Self {
        // Generate a keypair for signing the blocks
        let mut csprng = OsRng {};  // Use OsRng from rand::rngs
        let keypair = Keypair::generate(&mut csprng);

        let genesis_block = Block {
            hash: String::from("genesis"),
            previous_hashes: vec![],
            data: String::from("Genesis Block"),
            signature: vec![0; 64],  // Placeholder signature
        };
        let mut blocks = HashMap::new();
        blocks.insert(genesis_block.hash.clone(), genesis_block.clone());

        Self {
            blocks,
            genesis_block,
            keypair,
        }
    }

    pub async fn start(&mut self) {
        println!("N8IV Blockchain is up and running.");
    }

    pub fn add_block(&mut self, previous_hashes: Vec<String>, data: String) {
        let hash = self.generate_hash(&data);

        // Sign the data using the keypair
        let signature = self.keypair.sign(hash.as_bytes()).to_bytes().to_vec();

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
