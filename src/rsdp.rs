//! RSDP (Root System Description Pointer) model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RsdpSignature([u8; 8]);

impl RsdpSignature {
    pub fn new(sig: [u8; 8]) -> Self { Self(sig) }
    pub fn is_valid(&self) -> bool { &self.0 == b"RSD PTR " }
}

#[derive(Debug, Clone)]
pub struct Rsdp {
    pub signature: RsdpSignature,
    pub oem_id: [u8; 6],
    pub revision: u8,
    pub rsdt_address: u32,
    pub xsdt_address: u64,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rsdp_signature() {
        let sig = RsdpSignature::new(*b"RSD PTR ");
        assert!(sig.is_valid());
    }
}
