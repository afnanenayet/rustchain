/// # blockchain
///
/// The struct that represents the entire blockchain.
/// It stores the current blockchain as well as the transactions, also
/// contains helper functions related to managing the blockchain

use block::Block;
use transaction::Transaction;

#[derive(Debug)]
#[derive(Clone)]
pub struct Blockchain {
    chain: Vec<Block>,
    transactions: Vec<Transaction>,
}

impl Blockchain {
    /// Returns the number of nodes in the blockchain
    pub fn get_chain_len(&self) -> u64 {
        self.chain.len() as u64
    }

    /// Pushes a block onto to the blockchain, resets transactions in main 
    /// blockchain
    pub fn new_block(&mut self, proof: u64, previous_hash: Option<String>) {
        // Find previous hash if it wasn't provided (get it from the last element in the
        // array)
        let previous_hash = match previous_hash {
            Some(s) => s,
            None => {
                let ref last_block = self.chain[self.chain.len() - 1];
                last_block.prev_hash.clone()
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
}
