//! DSDT (Differentiated System Description Table) model.

#[derive(Debug, Clone)]
pub struct AmlBytecode {
    pub bytes: Vec<u8>,
}

#[derive(Debug, Clone)]
pub struct Dsdt {
    pub header: crate::table::SdtHeader,
    pub aml: AmlBytecode,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn dsdt_aml() {
        let d = Dsdt {
            header: crate::table::SdtHeader {
                signature: *b"DSDT",
                length: 200,
                revision: 2,
                oem_id: [0; 6],
                oem_table_id: [0; 8],
                oem_revision: 0,
                asl_compiler_id: [0; 4],
                asl_compiler_revision: 0,
            },
            aml: AmlBytecode { bytes: vec![0x10, 0x40] },
        };
        assert_eq!(&d.header.signature, b"DSDT");
    }
}
