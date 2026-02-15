#![cfg_attr(not(any(test, feature = "export-abi")), no_main)]

#[cfg(not(any(test, feature = "export-abi")))]
#[unsafe(no_mangle)]
pub extern "C" fn main() {}

#[cfg(feature = "export-abi")]
fn main() {
    stylus_sdk::abi::export::print_abi::<stylus_hardware_anchor::StylusHardwareAnchor>(
        "UNLICENSED",
        "pragma solidity ^0.8.0;",
    );
}
