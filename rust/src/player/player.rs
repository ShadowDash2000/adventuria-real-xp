use godot::builtin::{Vector2, Vector3, real};
use godot::classes::{
    Camera3D, CharacterBody3D, ICharacterBody3D, Input, InputEvent, InputEventMouseMotion,
};
use godot::global::godot_error;
use godot::obj::{Base, Gd, Singleton, WithBaseField};
use godot::register::{GodotClass, godot_api};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
struct Player {
    base: Base<CharacterBody3D>,

    #[export]
    speed: real,
    #[export]
    acceleration: f64,
    #[export]
    jump_impulse: real,
    #[export]
    gravity: real,
    #[export]
    mouse_sensitivity: real,
    #[export]
    joy_sensitivity: real,

    camera3d: Option<Gd<Camera3D>>,
    pitch: f32,
    camera_rotation: Vector2,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self {
            base,
            speed: 10.0,
            acceleration: 1000.0,
            jump_impulse: 100.0,
            gravity: -10.0,
            mouse_sensitivity: 0.01,
            joy_sensitivity: 1.0,
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
        let Some(camera) = self.base().try_get_node_as::<Camera3D>("Camera3D") else {
            godot_error!("Camera3D not found in Player node");
            return;
        };

        self.pitch = camera.get_rotation().x;
        self.camera3d = Some(camera);
    }

    fn input(&mut self, event: Gd<InputEvent>) {
        self.handle_mouse_rotation(event.clone());
    }
}

impl Player {
    fn rotate_camera(&mut self) {
        let new_rotation = self.camera_rotation;
        self.camera_rotation = Vector2::ZERO;

        if new_rotation.length() <= 0.0 {
            return;
        }

        self.base_mut().rotate_y(-new_rotation.x);

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
