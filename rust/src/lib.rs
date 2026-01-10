mod movable;
mod player;
mod state_machine;

use godot::prelude::{ExtensionLibrary, gdextension};

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
