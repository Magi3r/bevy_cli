//! All lints offered by `bevy_lint`.
//!
//! Click on each module to learn more about individual lints. Within each module is a static that
//! documents a lint's name, group, and short description, such as
//! [`missing_reflect::MISSING_REFLECT`].

use crate::lint::LintGroup;
use rustc_lint::LintStore;

pub mod complexity;
pub mod correctness;
pub mod nursery;
pub mod pedantic;
pub mod performance;
pub mod restriction;
pub mod style;
pub mod suspicious;

pub(crate) fn register_lints(store: &mut LintStore) {
    complexity::Complexity::register_lints(store);
    correctness::Correctness::register_lints(store);
    nursery::Nursery::register_lints(store);
    pedantic::Pedantic::register_lints(store);
    performance::Performance::register_lints(store);
    restriction::Restriction::register_lints(store);
    style::Style::register_lints(store);
    suspicious::Suspicious::register_lints(store);
}

pub(crate) fn register_passes(store: &mut LintStore) {
    complexity::Complexity::register_passes(store);
    correctness::Correctness::register_passes(store);
    nursery::Nursery::register_passes(store);
    pedantic::Pedantic::register_passes(store);
    performance::Performance::register_passes(store);
    restriction::Restriction::register_passes(store);
    style::Style::register_passes(store);
    suspicious::Suspicious::register_passes(store);
}

pub(crate) fn register_groups(store: &mut LintStore) {
    complexity::Complexity::register_group(store);
    correctness::Correctness::register_group(store);
    nursery::Nursery::register_group(store);
    pedantic::Pedantic::register_group(store);
    performance::Performance::register_group(store);
    restriction::Restriction::register_group(store);
    style::Style::register_group(store);
    suspicious::Suspicious::register_group(store);
}
