use godot::classes::{CharacterBody3D, ICharacterBody3D};
use godot::obj::Base;
use godot::register::{GodotClass, godot_api};

#[derive(GodotClass)]
#[class(base=CharacterBody3D)]
pub struct Player {
    base: Base<CharacterBody3D>,
}

#[godot_api]
impl ICharacterBody3D for Player {
    fn init(base: Base<Self::Base>) -> Self {
        Self { base }
    }
}
