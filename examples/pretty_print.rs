//! Demonstrates the `pretty` feature.
//!
//! Run with: `cargo run --example pretty_print --features pretty`

use pddl::{Name, Pretty, Type, Variable};

fn main() {
    let name = Name::new("location");
    let var = Variable::from("x");
    let simple = Type::from("truck");
    let either = Type::from_iter(["car", "truck", "motorbike"]);

    println!("== trivial cases (width 80) ==");
    println!("{}", name.pretty(80));
    println!("{}", var.pretty(80));
    println!("{}", simple.pretty(80));

    println!("\n== (either …) at different widths ==");
    for width in [80, 20, 12, 8] {
        println!("-- width = {width}");
        println!("{}", either.pretty(width));
    }
}
