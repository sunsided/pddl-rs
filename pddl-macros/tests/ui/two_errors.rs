static DOMAIN: std::sync::LazyLock<pddl::Domain> = pddl_macros::domain!(r#"
    (define (domain d)
      (:predicats (on ?s))
      (:action flip :parameters (?s -)))
"#);

fn main() {}
