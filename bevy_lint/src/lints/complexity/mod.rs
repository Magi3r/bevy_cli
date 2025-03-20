use rustc_lint::Level;

use crate::declare_bevy_group;

declare_bevy_group! {
    /// A group that offers suggestions on how to simplify your code.
    pub static COMPLEXITY = {
        name: "bevy::complexity",
        level: Level::Warn,
    };
}
