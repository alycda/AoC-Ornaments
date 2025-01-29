use std::str::FromStr;

#[derive(Debug, derive_more::Deref, derive_more::Display)]
pub struct AocHash(String);

impl FromStr for AocHash {
    type Err = miette::Error;

    /// .trim() because otherwise the /n will be included in the hash calculation (－‸ლ)
    fn from_str(input: &str) -> miette::Result<Self> {
        Ok(Self(input.trim().to_owned()))
    }
}

impl AocHash {
    pub fn generate_hash_bytes(key: &str, number: u32) -> [u8; 16] {
        let input = format!("{}{}", key, number);
        md5::compute(input).0
    }
    
    /// First two bytes must be 0
    pub fn has_five_leading_zeros(hash: &[u8; 16]) -> bool {
        hash[0] == 0 && hash[1] == 0 && (hash[2] & 0xf0) == 0
    }
    
    /// First three bytes must be 0
    pub fn has_six_leading_zeros(hash: &[u8; 16]) -> bool {
        hash[0] == 0 && hash[1] == 0 && hash[2] == 0
    }
}