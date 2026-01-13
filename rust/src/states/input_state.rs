use godot::classes::{Engine, INode, Node};
use godot::obj::{Base, Gd, Singleton, WithUserSignals};
use godot::prelude::{godot_api, Export, GodotClass, GodotConvert, Var};

#[derive(GodotConvert, Var, Export, Default, Copy, Clone, Debug, PartialEq)]
#[godot(via = u8)]
pub enum InputStates {
    #[default]
    Movement,
    InteractiveUI,
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct InputStateManager {
    base: Base<Node>,

    #[export]
    active_state: InputStates,
}

#[godot_api]
impl InputStateManager {
    pub const SINGLETON: &'static str = "InputState";

    #[signal]
    pub fn state_changed(new_state: InputStates);

    pub fn singleton() -> Gd<InputStateManager> {
        match Engine::singleton().get_singleton(Self::SINGLETON) {
            Some(singleton) => singleton.cast::<Self>(),
            None => panic!("Failed to find singleton -> {}", Self::SINGLETON),
        }
    }

    pub fn get_state(&self) -> InputStates {
        self.active_state
    }

    pub fn set_state(&mut self, new_state: InputStates) {
        self.active_state = new_state;
        self.signals().state_changed().emit(new_state);
    }
}

#[godot_api]
impl INode for InputStateManager {
    fn ready(&mut self) {
        let active_state = self.active_state;
        self.signals().state_changed().emit(active_state);
    }
}
