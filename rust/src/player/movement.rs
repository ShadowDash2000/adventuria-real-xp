use crate::movable::Movable3D;
use crate::states::{InputStateListener, InputStateManager, InputStates};
use godot::builtin::{real, Vector3};
use godot::classes::{CharacterBody3D, INode3D, Input, InputEvent, Node3D};
use godot::global::godot_error;
use godot::obj::{Base, Gd, Singleton, WithBaseField};
use godot::register::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(base=Node3D)]
struct PlayerMovement {
    base: Base<Node3D>,

    #[export]
    speed: real,
    #[export]
    acceleration: f64,
    #[export]
    jump_impulse: real,
    #[export]
    gravity: real,

    #[export]
    movement_node: Option<Gd<CharacterBody3D>>,
}

#[godot_api]
impl INode3D for PlayerMovement {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            speed: 10.0,
            acceleration: 1000.0,
            jump_impulse: 8.0,
            gravity: -10.0,
            movement_node: None,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.move_player(delta);
    }

    fn ready(&mut self) {
        if self.movement_node.is_none() {
            godot_error!("MovementNode not found in Player node");
        }

        self.connect_to_input_state();
        let input_state = InputStateManager::singleton();
        self.set_state_activity(input_state.bind().get_state());
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.handle_jump(event);
    }
}

impl InputStateListener for PlayerMovement {
    fn on_input_state_changed(&mut self, new_state: InputStates) {
        self.set_state_activity(new_state);
    }
}

impl PlayerMovement {
    fn set_state_activity(&mut self, state: InputStates) {
        let is_active = state == InputStates::Movement;
        self.set_events_activity(is_active);
    }

    fn set_events_activity(&mut self, is_active: bool) {
        self.base_mut().set_physics_process(is_active);
        self.base_mut().set_process_input(is_active);
    }

    fn move_player(&mut self, delta: f64) {
        let input = Input::singleton();
        let raw_input = input.get_vector("move_left", "move_right", "move_forward", "move_back");

        let Some(movement_node) = self.movement_node.clone() else {
            return;
        };
        let basis = movement_node.get_basis();
        let direction = basis * Vector3::new(raw_input.x, 0.0, raw_input.y);

        self.move_and_slide(movement_node, direction, delta);
    }

    fn handle_jump(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("jump") {
            if let Some(movement_node) = self.movement_node.clone() {
                self.jump(movement_node);
            }
        }
    }
}

impl Movable3D for PlayerMovement {
    fn get_speed(&self) -> real {
        self.speed
    }
    fn get_acceleration(&self) -> f64 {
        self.acceleration
    }
    fn get_jump_impulse(&self) -> real {
        self.jump_impulse
    }
    fn get_gravity(&self) -> real {
        self.gravity
    }
}
