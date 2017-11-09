/// REST frontend for the blockchain, so that it can be used and tested.
/// The endpoints are the following:
/// - `/chain` [GET]: returns the whole chain
/// - `/transaction/new`: [PUT] creates a new transaction
/// - `/mine`: has the server mine a new block

extern crate pencil;
extern crate rustchain;
extern crate serde_json;

use pencil::{
    Pencil, 
    Request, 
    Response, 
    PencilResult,
    jsonify,
    PathBound,
};

use rustchain::block::Block;
use rustchain::blockchain::Blockchain;
use rustchain::transaction::Transaction;

/// Mines a block from the blockchain
fn mine_endpoint(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}

/// Returns the entire chain as a JSON object
fn return_full_chain(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}

/// Creates a new transaction and adds it to the blockchain
fn create_new_transact(_: &mut Request) -> PencilResult {
    Ok(Response::from("Hello World!"))
}

fn main() {
    // Planning to use Pencil
    let mut app = Pencil::new("/web/rustchain");
    let blockchain = Blockchain::new();
    let mut file = app.open_resource("blockchain.json");

    // Register endpoints
    app.get("/mine", "mine", mine_endpoint);
    app.put("/transaction/new", "new transaction", create_new_transact);
    app.get("/chain", "chain", return_full_chain);
    app.run("127.0.0.1:5000");
}
