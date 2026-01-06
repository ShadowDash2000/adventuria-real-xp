mod player;
mod movable;
mod player_movement;

use godot::prelude::{ExtensionLibrary, gdextension};

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
