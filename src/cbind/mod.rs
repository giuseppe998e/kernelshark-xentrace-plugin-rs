// From "src/libkshark.h" & "src/libkshark-plugin.h" @ https://github.com/yordan-karadzhov/kernel-shark
pub(crate) mod kshark;

// From "xen/include/public/trace.h" @ https://github.com/xen-project/xen
pub(crate) mod xen;

// N.B. Use '(cargo clean) && (rustup run nightly cargo rustc -- -Zprint-type-sizes > sizes.txt)'
// to print the sizes of the project structures.
