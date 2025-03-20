use rustc_lint::{Level, Lint, LintStore};

use super::LintGroup;

pub struct Pedantic;

impl LintGroup for Pedantic {
    const NAME: &str = "bevy::pedantic";
    const LEVEL: Level = Level::Allow;
    const LINTS: &[&Lint] = &[];

    fn register_passes(_store: &mut LintStore) {
        todo!()
    }
}
