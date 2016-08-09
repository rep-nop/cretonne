//! RISC-V Settings.

// Include code generated by `meta/gen_settings.py`. This file contains a public `Settings` struct
// with an impl for all of the settings defined in `meta/cretonne/settings.py`.
include!(concat!(env!("OUT_DIR"), "/settings-riscv.rs"));