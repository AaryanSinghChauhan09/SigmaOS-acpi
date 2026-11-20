# SigmaOS-acpi Component Wiki

## Overview
SigmaOS-acpi provides ACPI (Advanced Configuration and Power Interface) table parsing primitives for the SigmaOS operating system. It demonstrates the hardware abstraction layer for system configuration and power management, following the principles of minimal kernel design and clear abstraction.

## Principles
- Single Responsibility
- Minimal Kernel Philosophy
- Fail-Fast on ACPI Table Errors
- Deterministic ACPI Initialization
- Clear Abstraction Over ACPI APIs
- Composition over Inheritance for ACPI implementations

## Architecture
SigmaOS-acpi follows an ACPI parsing pattern:
- **Table Parsing**: Provides routines to parse Differentiated System Description Table (DSDT), Multiple API Description Table (MADT), and other ACPI tables
- **Hardware Abstraction**: Abstracts hardware-specific details from the core OS
- **Power Management**: Handles basic power state transitions and configuration
- **Error Handling**: Fail-fast on malformed or unsupported ACPI tables

## API
The component provides the following public APIs:
- `acpi::table::parse(data)` - Parse raw ACPI table data
- `acpi::madt::parse(data)` - Parse Multiple API Description Table
- `acpi::find_table(signature)` - Find ACPI table by signature
- `acpi::get_oem_table()` - Get OEM-specific ACPI tables
- Error types for parsing failures or unsupported table versions

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific ACPI table data
2. **Act**: Execute the code under test (table parsing, signature lookup)
3. **Assert**: Verify the expected ACPI parsing results

Example:
```rust
#[test]
fn test_acpi_madt_parsing_aaa() {
    // Arrange
    let madt_data = /* valid MADT binary data */;
    
    // Act
    let madt = acpi::madt::parse(&madt_data);
    
    // Assert
    assert!(madt.is_ok());
    assert!(madt.unwalk().interrupt_controllers.len() > 0);
}
```

## Development
- Follows Rust best practices with emphasis on no_std compatibility
- ACPI initialization is best-effort; gracefully degrades on unsupported tables
- Comprehensive error types for all ACPI parsing failure scenarios
- Unit tests provided for table parsing and signature detection
- Integration tests for full ACPI initialization with real firmware tables

## TODO
- Add support for additional ACPI table versions and extensions
- Expand interrupt controller parsing capabilities
- Add power management state validation
- Create integration tests with real system firmware
- Implement ACPI thermal and clock resource parsing