use sha1::{Digest, Sha1};

pub struct Hasher;

impl Hasher{
    pub fn hash(content: &Vec<u8>) -> String{
        let size = content.len().to_string();
        let b = b"\0";

        let input = ["blob ".as_bytes(), size.as_bytes(), b, &content].concat();

        let mut sha1 = Sha1::new();
        sha1.update(input);

        let result_in_bytes = sha1.finalize().to_vec();
        let hash = result_in_bytes.iter().map(|byte| format!("{:02x}", byte)).collect::<String>();

        hash
    }
}