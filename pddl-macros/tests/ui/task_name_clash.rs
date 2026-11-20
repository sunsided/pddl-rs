#[pddl_macros::task(domain_source = r#"
    (define (domain d) (:predicates (on-table ?x) (on_table ?x)))
"#)]
mod d {}

fn main() {}
