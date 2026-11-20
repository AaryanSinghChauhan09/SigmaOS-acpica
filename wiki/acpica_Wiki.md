# SigmaOS-acpica Component Wiki

## Overview
SigmaOS-acpica provides ACPI Component Architecture (ACPI) support for the SigmaOS operating system. It demonstrates the ACPI subsystem and table parsing capabilities within the SigmaOS ecosystem, following the principles of minimal kernel design and deterministic initialization.

## Principles
- Single Responsibility
- ACPI Abstraction
- Fail-Fast on ACPI Initialization Errors
- Deterministic ACPI Setup
- Clear Abstraction Over ACPI APIs
- Composition over Inheritance for ACPI implementations

## Architecture
SigmaOS-acpica follows an ACPI pattern:
- **ACPI Core**: Provides ACPI table parsing, DSDT, and SSDT extraction primitives
- **Hardware Abstraction**: Abstracts hardware-specific ACPI operations for power management and device control
- **Intermediate Language**: Supports ACPI Bytecode execution and namespace traversal
- **Error Handling**: Fail-fast on invalid ACPI tables or unsupported configurations

## API
The component provides the following public APIs:
- `acpi::table::parse(data)` - Parse raw ACPI table data
- `acpi::dsdt::extract()` - Extract Differentiated System Description Table
- `acpi::ssdt::extract()` - Extract Secondary System Description Table
- `acpi::status()` - Get ACPI subsystem status and error information
- Error types for ACPI parsing failures or unsupported table versions

## Test Structure (Arrange-Act-ASSERT)
All tests follow the AAA pattern:
1. **Arrange**: Set up test environment with specific ACPI configuration
2. **Act**: Execute the code under test (ACPI table parsing, extraction)
3. **Assert**: Verify the expected ACPI results

Example:
```rust
#[test]
fn test_acpi_table_parse_aaa() {
    // Arrange
    let raw_data = include_bytes!("../test-data/test-acpi.bin");
    
    // Act
    let result = acpi::table::parse(raw_data);
    
    // Assert
    assert!(result.is_ok());
    let table = result.unwrap();
    assert!(table.is_valid());
}
```

## Development
- Follows Rust best practices with `#![forbid(unsafe_code)]`
- ACPI initialization is best-effort; gracefully degrades on invalid tables or unsupported hardware
- Comprehensive error types for all ACPI failure scenarios
- Unit tests provided for table parsing and extraction
- Integration tests for full ACPI with real system firmware

## TODO
- Add support for additional ACPI table versions and types
- Expand DSDT/SSDT extraction capabilities (power management, device probing)
- Add ACPI namespace traversal and method execution features
- Create integration tests with real system firmware
- Implement ACPI event handling and notification support