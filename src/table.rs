//! ACPI SDT header model.

#[derive(Debug, Clone)]
pub struct SdtHeader {
    pub signature: [u8; 4],
    pub length: u32,
    pub revision: u8,
    pub oem_id: [u8; 6],
    pub oem_table_id: [u8; 8],
    pub oem_revision: u32,
    pub asl_compiler_id: [u8; 4],
    pub asl_compiler_revision: u32,
}

#[derive(Debug, Clone)]
pub struct Table {
    pub header: SdtHeader,
    pub data: Vec<u8>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sdt_header() {
        let h = SdtHeader {
            signature: *b"FACP",
            length: 100,
            revision: 1,
            oem_id: [0; 6],
            oem_table_id: [0; 8],
            oem_revision: 0,
            asl_compiler_id: [0; 4],
            asl_compiler_revision: 0,
        };
        assert_eq!(&h.signature, b"FACP");
    }
}
