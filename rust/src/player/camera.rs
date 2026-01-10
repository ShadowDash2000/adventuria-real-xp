use crate::player::player::Player;
use crate::player::state::PlayerState;
use crate::player::state_controller::{PlayerStates};
use godot::builtin::{Vector2, Vector3, real};
use godot::classes::{Camera3D, INode, Input, InputEvent, InputEventMouseMotion, Node};
use godot::global::godot_error;
use godot::obj::{Base, Gd, Singleton, WithBaseField};
use godot::register::{GodotClass, godot_api};

#[derive(GodotClass)]
#[class(base=Node)]
struct PlayerCamera {
    base: Base<Node>,

    #[export]
    mouse_sensitivity: real,
    #[export]
    joy_sensitivity: real,

    player: Option<Gd<Player>>,
    camera3d: Option<Gd<Camera3D>>,
    pitch: f32,
    camera_rotation: Vector2,
}

#[godot_api]
impl INode for PlayerCamera {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            mouse_sensitivity: 0.01,
            joy_sensitivity: 1.0,
            player: None,
            camera3d: None,
            pitch: 0.0,
            camera_rotation: Vector2::ZERO,
        }
    }

    fn physics_process(&mut self, delta: f64) {
        self.handle_joypad_rotation(delta);
        self.rotate_camera();
    }

    fn ready(&mut self) {
        let Some(mut tree) = self.base().get_tree() else {
            return;
        };

        let Some(player) = tree
            .get_first_node_in_group("player")
            .and_then(|node| node.try_cast::<Player>().ok())
        else {
            godot_error!("Player node not found in group 'player' or wrong type");
            return;
        };

        let Some(camera) = tree
            .get_first_node_in_group("player")
            .and_then(|node| node.try_get_node_as::<Camera3D>("Camera3D"))
        else {
            godot_error!("Camera3D not found at expected path");
            return;
        };

        self.player = Some(player);

        self.pitch = camera.get_rotation().x;
        self.camera3d = Some(camera);

        self.connect_to_player_state();
        self.set_state_activity(false);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.handle_mouse_rotation(event.clone());
    }
}

impl PlayerState for PlayerCamera {
    fn on_player_state_changed(&mut self, new_state: PlayerStates) {
        let is_active = new_state == PlayerStates::Movement;
        self.set_state_activity(is_active);
    }
}

impl PlayerCamera {
    fn set_state_activity(&mut self, is_active: bool) {
        self.base_mut().set_physics_process(is_active);
        self.base_mut().set_process_input(is_active);
    }

    fn rotate_camera(&mut self) {
        let new_rotation = self.camera_rotation;
        self.camera_rotation = Vector2::ZERO;

        if new_rotation.length() <= 0.0 {
            return;
        }

        if let Some(mut player) = self.player.clone() {
            player.rotate_y(-new_rotation.x);
        }

        if let Some(mut camera) = self.camera3d.clone() {
            self.pitch = (self.pitch - new_rotation.y).clamp(-1.5, 1.5);

            camera.set_rotation(Vector3::new(self.pitch, 0.0, 0.0));
        }
    }

    fn handle_mouse_rotation(&mut self, event: Gd<InputEvent>) {
        if let Ok(mouse_motion) = event.try_cast::<InputEventMouseMotion>() {
            let rel = mouse_motion.get_relative();
            self.camera_rotation.x += rel.x * self.mouse_sensitivity;
            self.camera_rotation.y += rel.y * self.mouse_sensitivity;
        }
    }

    fn handle_joypad_rotation(&mut self, delta: f64) {
        let input = Input::singleton();
        let joypad = Vector2::new(
            input.get_axis("look_left", "look_right"),
            input.get_axis("look_up", "look_down"),
        );

        let joy_speed = 5.0;
        self.camera_rotation.x += joypad.x * joy_speed * delta as f32 * self.joy_sensitivity;
        self.camera_rotation.y += joypad.y * joy_speed * delta as f32 * self.joy_sensitivity;
    }
}
