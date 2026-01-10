use crate::movable::Movable3D;
use crate::state_machine::state::State;
use godot::builtin::{Vector3, real};
use godot::classes::input::MouseMode;
use godot::classes::{CharacterBody3D, INode3D, Input, InputEvent, Node3D};
use godot::global::godot_error;
use godot::obj::{Base, Gd, Singleton};
use godot::register::{GodotClass, godot_api, godot_dyn};

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

    fn ready(&mut self) {
        if self.movement_node.is_none() {
            godot_error!("MovementNode not found in Player node");
        }
    }
}

#[godot_dyn]
impl State for PlayerMovement {
    fn enter(&self) {
        let mut input = Input::singleton();
        input.set_mouse_mode(MouseMode::CAPTURED);
    }

    fn exit(&self) {}

    fn physics_process(&mut self, delta: f64) {
        self.move_player(delta);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.handle_jump(event);
    }
}

impl PlayerMovement {
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
