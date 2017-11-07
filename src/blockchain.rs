/// # blockchain
///
/// The struct that represents the entire blockchain. 
/// It stores the current blockchain as well as the transactions, also 
/// contains helper functions related to managing the blockchain

use block::Block;
use transaction::Transaction;

#[derive(Debug)]
pub struct Blockchain {
    chain: Vec<Block>,
    transactions: Vec<Transaction>,    
}

impl Blockchain {
    /// Returns the length of the blockchain
    pub fn get_chain_len(&self) -> u64 {
        self.chain.len() as u64
    }

    /// Adds a block to the blockchain
    pub fn add_block(&mut self, block: Block) {
        self.chain.push(block);
    }
}

