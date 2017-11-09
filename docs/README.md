# rustchain

A blockchain implementation written in Rust, based off of the tutorial
found [here](https://hackernoon.com/learn-blockchains-by-building-one-117428612f46)

## Build

This project was built with Cargo using the Rust stable compiler. To build:

    cargo build

To run:

    cargo run

## Dependencies

- serde

## Endpoint

There are three ways to interact with the binary via HTTP requests:

- `/transactions/new` creates a new transaction 
- `/mine` has the server mine a new block
- `/chain` returns the entire Blockchain
