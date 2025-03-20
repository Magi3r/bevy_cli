use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub struct Nursery;

impl LintGroup for Nursery {
    const NAME: &str = "bevy::nursery";
    const LEVEL: Level = Level::Allow;
    const LINTS: &[&Lint] = &[];

    fn register_passes(_store: &mut LintStore) {
        todo!()
    }
}
