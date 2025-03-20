use rustc_lint::Level;

use crate::declare_bevy_group;

declare_bevy_group! {
    /// A group of lints that encourage idiomatic code.
    ///
    /// These lints are opinionated and may be freely disabled if you disagree with their suggestions.
    pub static STYLE = {
        name: "bevy::style",
        level: Level::Warn,
    };
}
