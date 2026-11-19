static DOMAIN: std::sync::LazyLock<pddl::Domain> =
    pddl_macros::domain!("(define (problem p) (:domain d) (:init) (:goal (and)))");

fn main() {}
