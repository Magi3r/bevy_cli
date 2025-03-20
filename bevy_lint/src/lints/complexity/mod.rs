use rustc_lint::{Level, Lint, LintStore};

use super::LintGroup;

pub struct Complexity;

impl LintGroup for Complexity {
    const NAME: &str = "bevy::complexity";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[];

    fn register_passes(_store: &mut LintStore) {
        todo!()
    }
}
