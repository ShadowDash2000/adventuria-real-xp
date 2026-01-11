use crate::interaction::interaction::Interaction;
use godot::classes::Node;
use godot::global::godot_print;
use godot::obj::Base;
use godot::register::{GodotClass, godot_dyn};

#[derive(GodotClass)]
#[class(init, base=Node)]
struct TestInteraction {
    base: Base<Node>,
}

#[godot_dyn]
impl Interaction for TestInteraction {
    fn interact(&mut self) {
        godot_print!("Hello!");
    }
}
