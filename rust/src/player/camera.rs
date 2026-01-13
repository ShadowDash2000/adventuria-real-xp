use crate::player::player::Player;
use crate::states::{InputStateListener, InputStateManager, InputStates};
use godot::builtin::{real, Vector2, Vector3};
use godot::classes::{Camera3D, INode, Input, InputEvent, InputEventMouseMotion, Node};
use godot::global::godot_error;
use godot::obj::{Base, Gd, OnReady, Singleton, WithBaseField};
use godot::register::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(init, base=Node)]
struct PlayerCamera {
    base: Base<Node>,

    #[export]
    #[init(val = 0.01)]
    mouse_sensitivity: real,
    #[export]
    #[init(val = 1.0)]
    joy_sensitivity: real,

    #[init(val = OnReady::manual())]
    player: OnReady<Gd<Player>>,
    #[init(node = "%Camera3D")]
    camera3d: OnReady<Gd<Camera3D>>,
    pitch: f32,
    #[init(val = Vector2::ZERO)]
    camera_rotation: Vector2,
}

#[godot_api]
impl INode for PlayerCamera {
    fn physics_process(&mut self, delta: f64) {
        self.handle_joypad_rotation(delta);
        self.rotate_camera();
    }

    fn ready(&mut self) {
        let Some(mut tree) = self.base().get_tree() else {
            return;
        };

        match tree
            .get_first_node_in_group("player")
            .and_then(|node| node.try_cast::<Player>().ok())
        {
            Some(player) => self.player.init(player),
            None => godot_error!("Player node not found in group 'player' or wrong type"),
        }

        self.pitch = self.camera3d.get_rotation().x;

        self.connect_to_input_state();
        let input_state = InputStateManager::singleton();
        self.set_state_activity(input_state.bind().get_state());
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.handle_mouse_rotation(event.clone());
    }
}

impl InputStateListener for PlayerCamera {
    fn on_input_state_changed(&mut self, new_state: InputStates) {
        self.set_state_activity(new_state);
    }
}

impl PlayerCamera {
    fn set_state_activity(&mut self, state: InputStates) {
        let is_active = state == InputStates::Movement;
        self.set_events_activity(is_active);
    }

    fn set_events_activity(&mut self, is_active: bool) {
        self.base_mut().set_physics_process(is_active);
        self.base_mut().set_process_input(is_active);
    }

    fn rotate_camera(&mut self) {
        let new_rotation = self.camera_rotation;
        self.camera_rotation = Vector2::ZERO;

        if new_rotation.length() <= 0.0 {
            return;
        }

        self.player.clone().rotate_y(-new_rotation.x);

        self.pitch = (self.pitch - new_rotation.y).clamp(-1.5, 1.5);
        self.camera3d
            .clone()
            .set_rotation(Vector3::new(self.pitch, 0.0, 0.0));
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
