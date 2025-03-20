use rustc_lint::Level;

use crate::declare_bevy_group;

declare_bevy_group! {
    /// A group similar to [`CORRECTNESS`] that checks for suspicious or usually wrong code.
    ///
    /// The linted code may have been written intentionally, but should probably still be fixed.
    pub static SUSPICIOUS = {
        name: "bevy::suspicious",
        level: Level::Warn,
    };
}
