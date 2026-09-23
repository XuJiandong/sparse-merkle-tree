//! Measure compiled merkle proof sizes across different scenarios.
//!
//! For a tree holding `N` leaves, this example generates a hash-based merkle
//! proof for `M` queried leaves and reports:
//!   * the number of sibling nodes in the merkle path,
//!   * the size in bytes of the *compiled* proof (`CompiledMerkleProof`).
//!
//! The `CompiledMerkleProof` is the compact on-the-wire form consumed by
//! `verify`, so its byte length is the meaningful "proof size".
//!
//! # Performance
//!
//! Building the tree is by far the most expensive part (proof generation and
//! compilation take microseconds). The plain tree keeps a branch node at every
//! one of the 256 levels for each leaf, so a 100k-leaf tree materializes ~24M
//! branch nodes and the build takes ~9s. This example therefore builds against
//! the storage-optimized `trie` backend (shortcut nodes), which is enabled for
//! development targets via the crate's dev-dependencies, bringing the build
//! down to well under a second while producing the identical tree root, proof
//! and compiled-proof bytes.
//!
//! Run with:
//!   cargo run --example proof_size --release

use sparse_merkle_tree::{
    blake2b::{Blake2b, Blake2bBuilder, Blake2bHasher},
    default_store::DefaultStore,
    SparseMerkleTree, H256,
};

#[allow(clippy::upper_case_acronyms)]
type SMT = SparseMerkleTree<Blake2bHasher, H256, DefaultStore<H256>>;

/// Build a blake2b hasher used to derive deterministic keys/values.
fn hasher_for_key() -> Blake2b {
    Blake2bBuilder::new(32).personal(b"smt-pfsize").build()
}

/// Deterministically derive the `i`-th key as an `H256`.
fn key_at(i: u64) -> H256 {
    let mut hasher = hasher_for_key();
    hasher.update(&i.to_le_bytes());
    let mut buf = [0u8; 32];
    hasher.finalize(&mut buf);
    buf.into()
}

/// Deterministically derive the `i`-th value as an `H256`.
fn value_at(i: u64) -> H256 {
    let mut hasher = hasher_for_key();
    hasher.update(b"value");
    hasher.update(&i.to_le_bytes());
    let mut buf = [0u8; 32];
    hasher.finalize(&mut buf);
    buf.into()
}

/// Measure proof sizes for a tree with `n` leaves and `m` queried keys.
///
/// Returns `(sibling_count, compiled_bytes)`. Leaf keys are spread across the
/// full 256-bit keyspace (dense indices hashed through blake2b), exercising the
/// general sparse case.
fn measure(n: u64, m: usize) -> (usize, usize) {
    // Build the tree.
    let mut tree = SMT::default();
    for i in 0..n {
        tree.update(key_at(i), value_at(i)).expect("update");
    }

    // Query the first `m` keys.
    let query_keys: Vec<H256> = (0..m).map(|i| key_at(i as u64)).collect();

    let proof = tree.merkle_proof(query_keys.clone()).expect("merkle proof");
    let sibling_count = proof.merkle_path().len();
    let compiled = proof.compile(query_keys).expect("compile proof");

    (sibling_count, compiled.0.len())
}

fn main() {
    let scenarios: Vec<(u64, usize)> =
        vec![(100_000, 1), (100_000, 10), (100_000, 100), (100_000, 1000)];

    println!(
        "{:>10} {:>6} {:>10} {:>14}",
        "N (leaves)", "M", "siblings", "compiled(B)"
    );

    for (n, m) in scenarios {
        let (siblings, compiled) = measure(n, m);
        println!("{:>10} {:>6} {:>10} {:>14}", n, m, siblings, compiled);
    }
}
