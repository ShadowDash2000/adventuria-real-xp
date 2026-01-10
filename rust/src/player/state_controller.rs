use godot::classes::{INode, Node};
use godot::obj::{Base, WithUserSignals};
use godot::register::{Export, GodotClass, GodotConvert, Var, godot_api};

#[derive(GodotConvert, Var, Export, Default, Copy, Clone, Debug, PartialEq, Eq)]
#[godot(via = i64)]
pub enum PlayerStates {
    #[default]
    Movement,
    InteractiveUI,
}

#[derive(GodotClass)]
#[class(init, base=Node)]
pub struct PlayerStateController {
    base: Base<Node>,

    #[export]
    active_state: PlayerStates,
}

#[godot_api]
impl PlayerStateController {
    #[signal]
    pub fn state_changed(new_state: PlayerStates);
}

#[godot_api]
impl INode for PlayerStateController {
    fn ready(&mut self) {
        let active_state = self.active_state;
        self.signals().state_changed().emit(active_state);
    }
}

impl PlayerStateController {
    pub fn set_state(&mut self, new_state: PlayerStates) {
        self.active_state = new_state;
        self.signals().state_changed().emit(new_state);
    }
}
