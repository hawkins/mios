# mios

A purely educational exercise in creating an operating system in Rust.

Follows the incredible work on this blog: https://os.phil-opp.com/

## Building

Standalone executable:

```bash
cargo xbuild --target x86_64-mios.json
```

Bootable disk image:

```bash
bootimage build
```

## Running

```bash
bootimage run
```

## Dependencies

- rust nightly
- rust-src component
- cargo-xbuild crate
- qemu
- bootimage crate
