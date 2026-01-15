mod auth;
mod create_node;
mod interaction;
mod movable;
mod player;
mod pocketbase;
mod states;
mod test_action;

use crate::pocketbase::PocketBase;
use crate::states::InputStateManager;
use godot::classes::Engine;
use godot::global::godot_warn;
use godot::init::InitLevel;
use godot::obj::{NewAlloc, Singleton};
use godot::prelude::{gdextension, ExtensionLibrary};

struct RustExtension;

#[gdextension]
unsafe impl ExtensionLibrary for RustExtension {
    fn on_level_init(level: InitLevel) {
        match level {
            InitLevel::Servers => {
                let mut engine = Engine::singleton();
                engine.register_singleton(PocketBase::SINGLETON, &PocketBase::new_alloc());
            }
            InitLevel::Scene => {
                let mut engine = Engine::singleton();
                engine.register_singleton(
                    InputStateManager::SINGLETON,
                    &InputStateManager::new_alloc(),
                );
            }
            _ => (),
        }
    }

    fn on_level_deinit(level: InitLevel) {
        match level {
            InitLevel::Servers => {
                let mut engine = Engine::singleton();

                if let Some(pocketbase_singleton) = engine.get_singleton(PocketBase::SINGLETON) {
                    engine.unregister_singleton(PocketBase::SINGLETON);
                    pocketbase_singleton.free();
                } else {
                    godot_warn!(
                        "Failed to find & free singleton -> {}",
                        PocketBase::SINGLETON
                    );
                }
            }
            InitLevel::Scene => {
                let mut engine = Engine::singleton();

                if let Some(input_state_singleton) =
                    engine.get_singleton(InputStateManager::SINGLETON)
                {
                    engine.unregister_singleton(InputStateManager::SINGLETON);
                    input_state_singleton.free();
                } else {
                    godot_warn!(
                        "Failed to find & free singleton -> {}",
                        InputStateManager::SINGLETON
                    );
                }
            }
            _ => (),
        }
    }
}
