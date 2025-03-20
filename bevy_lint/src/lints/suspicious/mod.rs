use rustc_lint::{Level, Lint, LintStore};

use super::LintGroup;

pub struct Suspicious;

impl LintGroup for Suspicious {
    const NAME: &str = "bevy::suspicious";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[];

    fn register_passes(_store: &mut LintStore) {
        todo!()
    }
}
