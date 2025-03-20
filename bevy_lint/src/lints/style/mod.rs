use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub mod plugin_not_ending_in_plugin;

pub struct Style;

impl LintGroup for Style {
    const NAME: &str = "bevy::style";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[plugin_not_ending_in_plugin::PLUGIN_NOT_ENDING_IN_PLUGIN];

    fn register_passes(store: &mut LintStore) {
        store.register_late_pass(|_| {
            Box::new(plugin_not_ending_in_plugin::PluginNotEndingInPlugin::default())
        });
    }
}
