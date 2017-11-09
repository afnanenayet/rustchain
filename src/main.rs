/// REST frontend for the blockchain, so that it can be used and tested.
/// The endpoints are the following:
/// - `/chain` [GET]: returns the whole chain
/// - `/transaction/new`: [PUT] creates a new transaction
/// - `/mine`: has the server mine a new block

extern crate rustchain;
extern crate pencil;

use pencil::{
    Pencil, 
    Request, 
    Response, 
    PencilResult,
    jsonify
};

use rustchain::{block::Block, blockchain::Blockchain, transaction::Transaction};

/// Mines a block from the blockchain
fn mine_endpoint() {
}

/// Returns the entire chain as a JSON object
fn return_full_chain() {
}

/// Creates a new transaction and adds it to the blockchain
fn create_new_transact() {
}


fn main() {
    // Planning to use Pencil
    let mut app = Pencil::new("/web/rustchain");
}
