use sha2::{Sha256, Digest};

use base64::engine::Engine as _;
use base64::engine::general_purpose::STANDARD as BASE64;


pub type Hash32Bytes = [u8; 32];

pub fn hash_leaf(leaf_value: String) -> Hash32Bytes {
    let mut hasher = Sha256::new();
    hasher.update("leaf:");
    hasher.update(leaf_value);
    return hasher.finalize().to_vec().try_into().expect("can't convert to static array");
}

pub fn hash_internal(left: Hash32Bytes, right: Hash32Bytes) -> Hash32Bytes {
    let mut hasher = Sha256::new();
    hasher.update("node:");
    hasher.update(left);
    hasher.update(right);
    return hasher.finalize().to_vec().try_into().expect("can't convert to static array");
}

pub fn decode_hash(hash_base64: &str) -> Hash32Bytes {
    let hash_vec = BASE64.decode(hash_base64).expect("invalid hash");
    assert_eq!(hash_vec.len(), 32);
    let hash: Hash32Bytes = hash_vec.try_into().expect("can't convert to static array");
    hash
}