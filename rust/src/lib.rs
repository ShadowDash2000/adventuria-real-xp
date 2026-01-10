mod movable;
mod player;

use godot::prelude::{ExtensionLibrary, gdextension};

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {}
