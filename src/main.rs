/// REST frontend for the blockchain, so that it can be used and tested.
/// The endpoints are the following:
/// - `/chain` [GET]: returns the whole chain
/// - `/transaction/new`: [PUT] creates a new transaction
/// - `/mine`: has the server mine a new block

extern crate pencil;
extern crate rustchain;
extern crate serde_json;

#[macro_use]
extern crate lazy_static;

use std::sync::Mutex;

use pencil::{
    Pencil,
    Request,
    Response,
    PencilResult,
};

use rustchain::blockchain::{Blockchain, proof_of_work};
use rustchain::transaction::Transaction;
use std::collections::HashMap;

// Create static blockchain as a singleton for easy access in the rest of the
// app
lazy_static! {
    static ref BLOCKCHAIN: Mutex<Blockchain> = Mutex::new(Blockchain::new());
}

/// Mines a block from the blockchain
fn mine_endpoint(req: &mut Request) -> PencilResult {
    // Get proof from last block and calculate new proof
    println!("Mining block");
    let last_block = BLOCKCHAIN.lock().unwrap().get_last_block();
    let last_proof = last_block.unwrap().get_proof();
    let proof = proof_of_work(last_proof);

    // Use IP address as sender
    let remote_addr = req.remote_addr();
    let recip_str = format!("{}", remote_addr);

    // Reward for new proof
    let reward = Transaction {
        sender: String::from("blockchain"),
        recipient: String::from(recip_str),
        amount: 1.0f64,
    };

    // Add the transaction and the block, return the index of the block
    let transact_index = BLOCKCHAIN.lock().unwrap().push_transaction(reward.clone());
    BLOCKCHAIN.lock().unwrap().new_block(proof, None);

    // Create map so we can convert response to JSON
    let transact_json = serde_json::to_string(&reward);
    let mut resp = HashMap::new();
    resp.insert("proof", proof.to_string());
    resp.insert("index", transact_index.to_string());
    resp.insert("transaction", transact_json.unwrap());

    println!("Block successfully mined");
    Ok(Response::from(serde_json::to_string(&resp).unwrap()))
}

/// Returns the entire chain as a JSON object
fn return_full_chain(_: &mut Request) -> PencilResult {
    println!("Full blockchain requested");

    // Return all blocks in JSON format
    let json_str = BLOCKCHAIN.lock().unwrap().get_json();
    Ok(Response::from(json_str))
}

/// Creates a new transaction and adds it to the blockchain
fn create_new_transact(req: &mut Request) -> PencilResult {
    let json = req.get_json();

    if json.is_none() {
        Ok(Response::from("Invalid request"))
    } else {
        // Convert JSON data into a hashmap
        let json_data = json.clone().unwrap().to_string();
        let json_map: HashMap<String, Transaction> = serde_json::from_str(&json_data).unwrap();

        // Create new transaction from JSON data and push it onto the blockchain
        let new_transaction = json_map["transaction"].clone();

        // Push transaction onto blockchain and respond to user
        BLOCKCHAIN.lock().unwrap().push_transaction(new_transaction);
        println!("Added transaction");
        Ok(Response::from("Transaction processed"))
    }
}

fn main() {
    let mut app = Pencil::new("/web/rustchain");

    // Set port and IP here
    let ip = "127.0.0.1";
    let port = "8080";
    let addr = format!("{}:{}", ip, port);
    println!("Your app will run at {}", &addr);

    // Register endpoints
    app.get("/mine", "mine", mine_endpoint);
    app.put("/transaction/new", "new transaction", create_new_transact);
    app.get("/chain", "chain", return_full_chain);
    app.run(&addr);
}
