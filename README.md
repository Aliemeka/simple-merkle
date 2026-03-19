# Simple Merkle Tree

Simple Merkle Tree is a Rust implementation of a Merkle tree designed to verify the integrity of address-amount pairs, such as those found in token distribution or airdrop scenarios. It reads a list of wallet addresses and their associated amounts from a CSV file, hashes each pair using SHA-256, and constructs a binary Merkle tree from the resulting leaf nodes. The tree exposes the Merkle root, layer-by-layer structure, and proof generation with verification — allowing you to cryptographically prove that a specific address-amount pair is included in the dataset without revealing the full list.

To use this project, provide a `data.csv` file in the root directory. The file must have exactly two columns: `address` and `amount`. The first row must be the header `address,amount`. Each subsequent row should contain a wallet address (as a string) and a non-negative integer amount, separated by a comma, with no extra spaces or additional columns. Amounts must be whole numbers that fit within a 64-bit unsigned integer (`u64`). Here is an example of a valid file:

Example
**data.csv**

```
address,amount
0x5C88C720556f41B96885CfCa84458a3492b4839d,80
0x1234567890abcdef1234567890abcdef12345678,100
0xAb8483F64d9C6d1EcF9b849Ae677dD3315835cb2,150
```
