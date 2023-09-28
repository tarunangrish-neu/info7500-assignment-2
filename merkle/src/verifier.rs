use std::collections::VecDeque;
use std::fs;
use serde::{Deserialize, Serialize};
use crate::util;
use crate::util::{decode_hash, Hash32Bytes, hash_internal};

#[derive(Serialize, Deserialize, Debug)]
struct MerkleProof {
    leaf_position: u64,
    leaf_value: String,
    proof_hash_values_base64: Vec<String>,
    proof_hash_values: Option<Vec<Hash32Bytes>>
}

pub fn run(proof_file: &String) {
    let merkle_proof = read_merkle_proof(proof_file);
    verify_merkle_proof(merkle_proof)
}

fn read_merkle_proof(proof_file: &String) -> Box<MerkleProof> {
    let contents = fs::read_to_string(proof_file)
        .expect("Something went wrong reading the file");

    // Deserialize the YAML content into a Config struct
    let mut merkle_proof: Box<MerkleProof> = serde_yaml::from_str(&contents)
        .expect("Error parsing the YAML file");

    assert!(merkle_proof.proof_hash_values.is_none());

    let mut decoded_hash = Vec::new();
    for hash_base64 in &merkle_proof.proof_hash_values_base64 {
        decoded_hash.push(decode_hash(hash_base64));
    }
    merkle_proof.proof_hash_values = Some(decoded_hash);
    merkle_proof
}

fn verify_merkle_proof(merkle_proof: Box<MerkleProof>) {
    let root = decode_hash("1qIbsvuF6FrhNjMD4p06srUye6G4FfFINDDkNfKUpTs=");
    let computed_root = compute_merkle_root_from_merkle_proof(merkle_proof);
    println!("computed_root: {:?}", computed_root);
    assert_eq!(computed_root, root);
}

fn compute_merkle_root_from_merkle_proof(merkle_proof: Box<MerkleProof>) -> util::Hash32Bytes {
    let mut pos = merkle_proof.leaf_position;
    let mut hashes = VecDeque::from(merkle_proof.proof_hash_values.unwrap());
    let mut cur_root = util::hash_leaf(merkle_proof.leaf_value);
    while let Some(hash) = hashes.pop_front() {
        let (left, right);
        if pos % 2 == 0 {
            left = cur_root;
            right = hash;
        } else {
            left = hash;
            right = cur_root;
        }
        cur_root = hash_internal(left, right);
        pos >>= 1;
    }
    cur_root
}
