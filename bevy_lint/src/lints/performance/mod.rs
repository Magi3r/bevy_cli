use rustc_lint::Level;

use crate::declare_bevy_group;

declare_bevy_group! {
    /// A group that suggests how to increase the performance of your code.
    pub static PERFORMANCE = {
        name: "bevy::performance",
        level: Level::Warn,
    };
}
