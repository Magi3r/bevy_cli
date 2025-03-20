use rustc_lint::Level;

use crate::declare_bevy_group;

declare_bevy_group! {
    /// A group of lints that make the linter incredibly nit-picky.
    ///
    /// If you enable this group, expect to liberally apply `#[allow(...)]` attributes throughout your
    /// code.
    pub static PEDANTIC = {
        name: "bevy::pedantic",
        level: Level::Allow,
    };
}
