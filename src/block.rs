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

#[derive(Debug)]
pub struct Block {
    index: u64, // index in the blockchain
    proof: u64,
    prev_hash: String, // hash of the previous block
}

