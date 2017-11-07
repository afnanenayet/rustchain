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

use blockchain::Blockchain;
use std::time::{SystemTime, UNIX_EPOCH, Duration};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};

#[derive(Debug)]
#[derive(Hash)]
pub struct Block {
    index: usize, // index in the blockchain
    proof: u64,
    prev_hash: String, // hash of the previous block
    timestamp: Duration,
}

impl Block {
    fn new(bchain: &Blockchain, proof: u64, prev_hash: String) -> Block {
        let sys_time = SystemTime::now();
        let u_time = sys_time.duration_since(UNIX_EPOCH).expect("Negatively elapsed time");

        Block {
            index: bchain.get_chain_len() + 1,
            prev_hash: prev_hash,
            proof: proof,
            timestamp: u_time,
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


