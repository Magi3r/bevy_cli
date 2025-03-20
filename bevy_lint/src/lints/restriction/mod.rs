use rustc_lint::{Level, Lint, LintStore};

use super::LintGroup;

pub struct Restriction;

impl LintGroup for Restriction {
    const NAME: &str = "bevy::restriction";
    const LEVEL: Level = Level::Allow;
    const LINTS: &[&Lint] = &[];

    fn register_passes(_store: &mut LintStore) {
        todo!()
    }
}
