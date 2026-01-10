mod movable;
mod player;

use godot::classes::Engine;
use godot::global::godot_warn;
use godot::init::InitLevel;
use godot::obj::{NewAlloc, Singleton};
use godot::prelude::{ExtensionLibrary, gdextension};
use godot_tokio::AsyncRuntime;

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {
    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let mut engine = Engine::singleton();
                engine.register_singleton(AsyncRuntime::SINGLETON, &AsyncRuntime::new_alloc());
            }
            _ => (),
        }
    }

    fn on_level_deinit(level: InitLevel) {
        match level {
            InitLevel::Scene => {
                let mut engine = Engine::singleton();

                if let Some(async_singleton) = engine.get_singleton(AsyncRuntime::SINGLETON) {
                    engine.unregister_singleton(AsyncRuntime::SINGLETON);
                    async_singleton.free();
                } else {
                    godot_warn!(
                        "Failed to find & free singleton -> {}",
                        AsyncRuntime::SINGLETON
                    );
                }
            }
            _ => (),
        }
    }
}
