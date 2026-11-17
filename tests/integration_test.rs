// Integration test for SigmaOS-tools-r-ds
// Verifies the crate builds and links as a dependency.
#![allow(unused_imports)]

use tools_r_ds as _;

#[test]
fn crate_links_and_runs() {
    assert_eq!(2 + 2, 4);
}
