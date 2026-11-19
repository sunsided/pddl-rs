#[pddl_macros::task(domain_source = r#"
    (define (domain d) (:requirements :derived-predicates)
      (:predicates (p ?x) (q ?x))
      (:derived (p ?x) (not (p ?x))))
"#)]
mod d {}

fn main() {}
