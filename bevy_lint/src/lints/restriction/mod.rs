use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub mod missing_reflect;
pub mod panicking_methods;

pub struct Restriction;

impl LintGroup for Restriction {
    const NAME: &str = "bevy::restriction";
    const LEVEL: Level = Level::Allow;
    const LINTS: &[&Lint] = &[
        missing_reflect::MISSING_REFLECT,
        panicking_methods::PANICKING_METHODS,
    ];

    fn register_passes(store: &mut LintStore) {
        store.register_late_pass(|_| Box::new(missing_reflect::MissingReflect::default()));
        store.register_late_pass(|_| Box::new(panicking_methods::PanickingMethods::default()));
    }
}
