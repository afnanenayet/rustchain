/// # blockchain
///
/// The struct that represents the entire blockchain.
/// It stores the current blockchain as well as the transactions, also
/// contains helper functions related to managing the blockchain

use block::Block;
use transaction::Transaction;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

/// A simple blockchain that contains a vector of transactions and a vector of Blocks/nodes.
/// Also contains related functions that manage the blockchain and its blocks.
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Blockchain {
    chain: Vec<Block>,
    transactions: Vec<Transaction>,
}

impl Blockchain {
    /// Creates a new blockchain with a genesis block
    pub fn new() -> Blockchain {
        // Create genesis block
        let block = Block::new(0, 100, 1, Vec::new());

        // Create empty blockchain and add the genesis block
        let mut bchain = Blockchain::default();
        bchain.chain.push(block);
        bchain
    }

    /// Returns the number of nodes in the blockchain
    pub fn get_chain_len(&self) -> u64 {
        self.chain.len() as u64
    }

    /// Returns the most recent block that was added to the chain, if there
    /// is at least one block in the chain.
    fn get_last_block(&self) -> Option<Block> {
        if self.chain.len() > 1 {
            Some(self.chain[self.chain.len() - 1].clone())
        } else {
            None
        }
    }

    /// Accepts a `Transaction` that will be added to the next mined block in
    /// the blockchain. Function returns the index of the `Block` that will
    /// hold this transaction
    pub fn push_transaction(&mut self, transaction: Transaction) -> u64 {
        self.transactions.push(transaction);

        if self.chain.len() == 0 {
            0
        } else {
            let last_block = self.get_last_block().unwrap();
            last_block.get_index() + 1
        }
    }

    /// Pushes a block onto to the blockchain, resets transactions in main
    /// blockchain
    pub fn new_block(&mut self, proof: u64, previous_hash: Option<u64>) {
        // Find previous hash if it wasn't provided (get it from the last element in the
        // array)
        let previous_hash = match previous_hash {
            Some(i) => i,
            None => {
                let ref last_block = self.get_last_block().unwrap();
                last_block.get_prev_hash()
            }
        };

        // Create the new block
        let block = Block::new(
            self.chain.len() as u64 + 1,
            proof,
            previous_hash,
            self.transactions.clone(),
        );

        // Push the block onto the blockchain
        self.chain.push(block);

        // Reset the transactions
        self.transactions = Vec::new();
    }

    /// A proof of work algorithm
    ///
    /// Define a number `p` such that `p` = proof and `p'` such that `p'` is
    /// the proof of work in the previous block. The function finds a number
    /// `p'` such that hashing "pp`" contains 4 leading zeroes.
    pub fn proof_of_work(&self, last_proof: u64) -> u64 {
        let mut proof = 0;

        while !valid_proof(last_proof, proof) {
            proof += 1;
        }
        return proof;
    }
}

/// Validates a potential proof, returning whether hashing `"pp'"` contains
/// 4 leading zeroes
pub fn valid_proof(proof_prime: u64, proof: u64) -> bool {
    // Concatenate proofs into one string
    let guess_str = format!("{}{}", proof_prime, proof);

    // Hash the string
    let mut hasher = DefaultHasher::new();
    guess_str.hash(&mut hasher);

    // Convert hash back into string
    let candidate_hash = hasher.finish().to_string();

    // Return whether last 4 characters of the string are zeroes
    &candidate_hash[candidate_hash.len()-5..] == "0000"
}

