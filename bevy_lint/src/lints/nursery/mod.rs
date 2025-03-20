use rustc_lint::Level;

use crate::declare_bevy_group;

declare_bevy_group! {
    /// A group of unstable lints that may be removed at any time for any reason.
    pub static NURSERY = {
        name: "bevy::nursery",
        level: Level::Allow,
    };
}
