use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub struct Performance;

impl LintGroup for Performance {
    const NAME: &str = "bevy::performance";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[];

    fn register_passes(_store: &mut LintStore) {
        todo!()
    }
}
