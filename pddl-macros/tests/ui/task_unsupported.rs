#[pddl_macros::task(domain_source = r#"
    (define (domain timed) (:requirements :strips :durative-actions))
"#)]
mod timed {}

fn main() {}
