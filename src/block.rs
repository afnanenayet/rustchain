/// # Block
///
/// A `Block` represents an entry in the blockchain
///
/// ## Synopsis
///
/// A block contains the following:
/// - an index (the index of the node)
/// - a list of transactions
/// - proof
/// - a hash of the previous block

use transaction::Transaction;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
#[derive(Hash)]
#[derive(Clone)]
pub struct Block {
    index: u64, // index in the blockchain
    proof: u64,
    pub prev_hash: String, // hash of the previous block
    timestamp: Duration, // time at which block was created
    transactions: Vec<Transaction>,
}

impl Block {
    /// Creates a new block with a reference to the whole blockchain, given proof, 
    /// optionally the hash of the previous blockhain, and proof. It will create 
    /// a new block with a timestamp and index with the information given
    pub fn new(index: u64, proof: u64, prev_hash: String, transactions: Vec<Transaction>) -> Block {
        let sys_time = SystemTime::now();
        let u_time = sys_time.duration_since(UNIX_EPOCH).expect("Negatively elapsed time");

        // the newly constructed node
        Block {
            index: index,
            prev_hash: prev_hash,
            proof: proof,
            timestamp: u_time,
            transactions: transactions,
        }
    }
}

/// Hashes a block, not placed in struct because blocks can't hash themselves
/// (needs the previous block)
fn hash(block: &Block) -> u64 {
    let mut hasher = DefaultHasher::new();
    block.hash(&mut hasher);
    hasher.finish()
}

