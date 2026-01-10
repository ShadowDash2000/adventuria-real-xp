use crate::state_machine::state::State;
use godot::classes::InputEvent;
use godot::prelude::*;

#[derive(GodotConvert, Var, Export, Default, Copy, Clone)]
#[godot(via = i64)]
enum States {
    #[default]
    Movement,
    InteractiveUI,
}

#[derive(GodotClass)]
#[class(init, base=Node)]
struct PlayerState {
    base: Base<Node>,

    #[export]
    default_state: States,

    #[export]
    movement_state: Option<DynGd<Node, dyn State>>,
    #[export]
    interactive_ui_state: Option<DynGd<Node, dyn State>>,

    active_state: Option<DynGd<Node, dyn State>>,
}

#[godot_api]
impl INode for PlayerState {
    fn physics_process(&mut self, delta: f64) {
        if let Some(active_state) = &mut self.active_state {
            active_state.dyn_bind_mut().physics_process(delta);
        }
    }

    fn ready(&mut self) {
        self.set_state(self.default_state);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        if let Some(active_state) = &mut self.active_state {
            active_state.dyn_bind_mut().input(event);
        }
    }
}

impl PlayerState {
    fn set_state(&mut self, state: States) {
        let Some(mut node_state) = (match state {
            States::Movement => self.movement_state.clone(),
            States::InteractiveUI => self.interactive_ui_state.clone(),
        }) else {
            return;
        };

        if let Some(active_state) = &mut self.active_state {
            active_state.dyn_bind_mut().exit();
        }

        node_state.dyn_bind_mut().enter();

        self.active_state = Some(node_state);
    }
}
