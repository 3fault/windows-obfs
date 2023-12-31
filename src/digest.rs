#[const_trait]
pub trait Digest {
    const OUTPUT_LEN: usize;
    fn digest(input: &[u8]) -> [u8; Self::OUTPUT_LEN];
}
