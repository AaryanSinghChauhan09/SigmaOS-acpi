//! FADT (Fixed ACPI Description Table) model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PowerManagementProfile { Unspecified, Desktop, Mobile, Workstation, Enterprise, Soho, Appliance }

#[derive(Debug, Clone)]
pub struct Fadt {
    pub header: crate::table::SdtHeader,
    pub firmware_ctrl: u32,
    pub dsdt_address: u32,
    pub pm1a_event_block: u32,
    pub pm1b_event_block: u32,
    pub pm1a_control_block: u32,
    pub pm1b_control_block: u32,
    pub pm2_control_block: u32,
    pub pm_timer_block: u32,
    pub profile: PowerManagementProfile,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fadt_profile() {
        let p = PowerManagementProfile::Mobile;
        assert_eq!(p, PowerManagementProfile::Mobile);
    }
}
