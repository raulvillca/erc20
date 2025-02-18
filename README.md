# Casper ERC-20

A library for developing ERC-20 tokens for the Casper Network.

The main functionality is provided via the ERC-20 struct, and is intended to be consumed by a smart contract written to be deployed on the Casper Network.

## Usage

To create an example ERC-20 contract which uses this library, use the `cargo-casper` tool:

```
cargo install cargo-casper
cargo casper --erc20 <PATH TO NEW PROJECT>
```

This command will generate a new project structure with an example token contract based on an [example project](example/erc20-token/src/main).

## Development

Make sure the `wasm32-unknown-unknown` Rust target is installed.

```
make prepare
```

## Build Smart Contracts
To build the example ERC-20 contract and supporting test contracts:

```
make build-contracts
```

## Test

```
make test
```

## JavaScript Client SDK

A [JavaScript client SDK](https://github.com/casper-network/casper-contracts-js-clients/tree/master/packages/erc20-client) can be used to interact with the ERC-20 contract. 


## Documentation

For more information, visit the below guides:
-  [ERC-20 Tutorial](TUTORIAL.md) - An illustrated guide on how to implement, deploy, and test an ERC-20 contract. 
-  [ERC-20 How-To Guide](https://casper.network/docs/workflow/erc-20-sample-guide) - An example-driven guide on how to setup, query, transfer, approve, and check the balance of an ERC-20 contract.


