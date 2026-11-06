//! ACPI device enumeration model.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AcpiDeviceType { Processor, Thermal, PowerButton, SleepButton, EmbeddedController, Other }

#[derive(Debug, Clone)]
pub struct AcpiDevice {
    pub hid: String,
    pub uid: String,
    pub device_type: AcpiDeviceType,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn acpi_device() {
        let d = AcpiDevice { hid: "PNP0C0C".into(), uid: "1".into(), device_type: AcpiDeviceType::PowerButton };
        assert_eq!(d.device_type, AcpiDeviceType::PowerButton);
    }
}
