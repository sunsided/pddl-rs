const PDDL: &str = "(define (domain d))";

static DOMAIN: std::sync::LazyLock<pddl::Domain> = pddl_macros::domain!(PDDL);

fn main() {}
