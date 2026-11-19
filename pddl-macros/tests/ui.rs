//! Compile-fail tests for the error messages. The `.stderr` files depend on the
//! compiler version, so CI runs these on stable Linux only, not on the MSRV.

#[test]
#[cfg_attr(
    not(target_os = "linux"),
    ignore = "the `.stderr` files contain `/` path separators and Linux rustc output"
)]
fn ui() {
    trybuild::TestCases::new().compile_fail("tests/ui/*.rs");
}
