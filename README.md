# SigmaOS-acpi

ACPI table parsing primitives for SigmaOS.

## Current boundary

RSDP, RSDT, XSDT, and common header parsing validate signatures, declared
lengths, little-endian fields, and checksums against caller-provided slices.
Physical-memory discovery is not implemented: `find_rsdp` and DSDT extraction
return no result until SigmaOS provides a safe firmware table mapper. FADT
field decoding is not implemented, so `parse_facp` returns no result rather
than fabricating placeholder values. These types are not yet suitable for
power-management decisions.

## Development

```sh
cargo check --lib
```
