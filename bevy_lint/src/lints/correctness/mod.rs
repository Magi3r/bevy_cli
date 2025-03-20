use rustc_lint::{Level, Lint, LintStore};

use super::LintGroup;

pub struct Correctness;

impl LintGroup for Correctness {
    const NAME: &str = "bevy::correctness";
    const LEVEL: Level = Level::Deny;
    const LINTS: &[&Lint] = &[];

    fn register_passes(_store: &mut LintStore) {
        todo!()
    }
}
