//! Generates the gripper domain with `pddl-codegen`, the way a build script
//! user would, so the build-script path is tested next to the attribute.

use pddl_core::{Domain, Parser};

fn main() {
    let path = "../tests/data/sas-plus/gripper-2/domain.pddl";
    println!("cargo::rerun-if-changed={path}");
    let source = std::fs::read_to_string(path).expect("the gripper domain exists");
    let domain = Domain::from_str(&source).expect("the gripper domain parses");
    let code = pddl_codegen::lifted(&domain).expect("the gripper domain is supported");
    let out =
        std::path::Path::new(&std::env::var("OUT_DIR").expect("set by Cargo")).join("gripper.rs");
    std::fs::write(out, code.to_string()).expect("OUT_DIR is writable");
}
