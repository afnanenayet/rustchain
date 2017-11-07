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
    
}

/// hashes a given block
fn hash(block: &Block) {
}
