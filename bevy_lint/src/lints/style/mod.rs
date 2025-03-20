use rustc_lint::{Level, Lint, LintStore};

use super::LintGroup;

pub struct Style;

impl LintGroup for Style {
    const NAME: &str = "bevy::style";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[];

    fn register_passes(_store: &mut LintStore) {
        todo!()
    }
}
