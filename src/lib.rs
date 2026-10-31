//! SigmaOS ACPI
//! ACPI power management

#![no_std]

pub mod acpi;

pub use acpi::{
    AcpiDeviceType, AcpiHeader, AcpiPowerState, AcpiUtility, Dsdt, Facp, Rsdp, Rsdt, Xsdt,
};
