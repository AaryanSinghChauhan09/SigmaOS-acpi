//! SigmaOS-acpi — ACPI model.
//!
//! Models ACPI tables (RSDP, RSDT, XSDT, FADT, DSDT, MADT),
//! power management states (S0-S5), device enumeration, and
//! AML interpretation basics. Deterministic and std-only
//! (no actual hardware access).

pub mod rsdp;
pub mod table;
pub mod fadt;
pub mod dsdt;
pub mod power;
pub mod device;

pub use rsdp::{Rsdp, RsdpSignature};
pub use table::{SdtHeader, Table};
pub use fadt::{Fadt, PowerManagementProfile};
pub use dsdt::{Dsdt, AmlBytecode};
pub use power::{PowerState, SleepState};
pub use device::{AcpiDevice, AcpiDeviceType};
