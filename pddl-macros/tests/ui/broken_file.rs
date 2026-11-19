static DOMAIN: std::sync::LazyLock<pddl::Domain> =
    pddl_macros::domain_file!("../fixtures/broken-domain.pddl");

fn main() {}
