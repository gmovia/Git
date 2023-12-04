use sha1::{Digest, Sha1};
use std::fmt::Write as FmtWrite;

pub struct Hasher;

impl Hasher {
    pub fn hash(input: &Vec<u8>) -> String {
        let mut sha1 = Sha1::new();
        sha1.update(input);

        let result_in_bytes = sha1.finalize().to_vec();
        let hash: String = result_in_bytes.iter().fold(String::new(), |mut acc, byte| {
            write!(acc, "{:02x}", byte).expect("Failed to write to String");
            acc
        });
        hash
    }
}
