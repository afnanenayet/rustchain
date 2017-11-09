# rustchain

A blockchain implementation written in Rust, based off of the tutorial
found [here](https://hackernoon.com/learn-blockchains-by-building-one-117428612f46)

## Build

This project was built with Cargo using the Rust stable compiler. To build:

    cargo build

To run:

    cargo run

## Endpoint

There are three ways to interact with the binary via HTTP requests:

- `/transactions/new` creates a new transaction
- `/mine` has the server mine a new block
- `/chain` returns the entire Blockchain

## Usage

This webapp runs as a REST endpoint. Here are the three methods available:

### `/transactions/new`

Accepts a JSON dictionary in the format `"transaction" : {JSON serialized
Transaction struct}`

If you want to do this manually, this is what the JSON request should look like:

    { "transaction":
        {
            "sender":"a string",
            "recipient":"a string",
            "amount":<a float>
        }
    }

### `/chain`

This will return the entire blockchain serialized in JSON with `serde-json`

### `/mine`

This will mine a new block and return the following information:

- the transaction awarded (`serde-json` serialized `Transaction` struct)
- the index of the Block/node in the blockchain
- the proof for the block you mined

