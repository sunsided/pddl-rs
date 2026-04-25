// ActionDefinition, DurativeActionDefinition, DerivedPredicate, StructureDef, StructureDefs,
// and DurativeActionGoalDefinition visitor impls are temporarily disabled due to RcDoc
// lifetime variance issues with optional/boxed fields.

#[cfg(test)]
mod tests {
    use crate::pretty_print::PrettyRenderer;

    #[test]
    fn placeholder() {
        let _ = PrettyRenderer;
    }
}
