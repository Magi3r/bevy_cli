use rustc_lint::Level;

use crate::declare_bevy_group;

declare_bevy_group! {
    /// A group of deny-by-default lints that check for outright wrong or useless code.
    ///
    /// These lints are carefully picked to be free of false positives. You should avoid
    /// `#[allow(...)]`-ing these lints without a _very_ good reason.
    pub static CORRECTNESS = {
        name: "bevy::correctness",
        level: Level::Deny,
    };
}
