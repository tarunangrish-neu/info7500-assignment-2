use std::fs;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
struct MerkleProof {
    leaf_position: u64,
    leaf_value: String,
    proof_hash_values: Vec<String>
}

pub fn run(proof_file: &String) {
    let merkle_proof = read_merkle_proof(proof_file);
    verify_merkle_proof(&merkle_proof)
}

fn read_merkle_proof(proof_file: &String) -> Box<MerkleProof> {
    let contents = fs::read_to_string(proof_file)
        .expect("Something went wrong reading the file");

    // Deserialize the YAML content into a Config struct
    let merkle_proof: Box<MerkleProof> = serde_yaml::from_str(&contents)
        .expect("Error parsing the YAML file");

    println!("{:?}", merkle_proof);
    merkle_proof
}

fn verify_merkle_proof(_proof: &MerkleProof) {

}
