use rustc_lint::{Level, Lint, LintStore};

use crate::lint::LintGroup;

pub mod insert_event_resource;
pub mod insert_unit_bundle;

pub struct Suspicious;

impl LintGroup for Suspicious {
    const NAME: &str = "bevy::suspicious";
    const LEVEL: Level = Level::Warn;
    const LINTS: &[&Lint] = &[
        insert_event_resource::INSERT_EVENT_RESOURCE,
        insert_unit_bundle::INSERT_UNIT_BUNDLE,
    ];

    fn register_passes(store: &mut LintStore) {
        store.register_late_pass(|_| {
            Box::new(insert_event_resource::InsertEventResource::default())
        });
        store.register_late_pass(|_| Box::new(insert_unit_bundle::InsertUnitBundle::default()));
    }
}
