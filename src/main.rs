mod hash_func;
mod merkle;
mod read_csv;

use crate::merkle::MerkleTree;
use crate::read_csv::read_csv;

fn verify_transaction(address: &str, amount: u64, merkle_tree: &MerkleTree) {
    let proof = merkle_tree.proof_block(address, amount);

    match proof {
        Some(proof) => {
            println!("Proof for block ({address}: {amount}):");
            let verified = merkle_tree.verify_proof(&address, amount, proof);
            println!("Proof verification result: {}", verified);
        }
        None => println!("Block not found in the Merkle Tree."),
    }
}

fn main() {
    let blocks = match read_csv("data.csv") {
        Ok(blocks) => blocks,
        Err(e) => {
            eprintln!("Error reading CSV: {}", e);
            return;
        }
    };

    let block_clone = blocks.clone();

    let merkle_tree = MerkleTree::build(blocks);

    let depth = merkle_tree.get_depth();
    println!("Generated a Merkle Tree with depth: {depth}");

    // Show merkle tree layers
    merkle_tree.print_tree();
    println!();

    // Merkle tree blocks
    let blocks_in_tree: Vec<String> = merkle_tree
        .blocks
        .iter()
        .map(|(address, amount)| format!("{}:{}", address, amount))
        .collect();

    println!("Blocks in Merkle Tree:");
    for (i, block) in blocks_in_tree.iter().enumerate() {
        let index = i + 1;
        println!("Block {index}: {block}");
    }

    // Merkle root
    let merkle_root = merkle_tree
        .get_root()
        .unwrap_or_else(|| "No root found".to_string());
    println!("\nMerkle Root: {}", merkle_root);

    // Proof inclusion for a specific block
    // Get random block from the blocks
    let target_block = block_clone[0].clone();
    let (address, amount) = target_block;

    println!("\nGenerating proof for block ({address}: {amount})");

    verify_transaction(&address, amount, &merkle_tree);

    // Proof for a non-existent block
    let fake_address = "0xdeefcabedef1234567890abcdef12345678";
    let fake_amount = 999;
    println!("\nGenerating proof for non-existent block ({fake_address}: {fake_amount})");

    verify_transaction(fake_address, fake_amount, &merkle_tree);
}
