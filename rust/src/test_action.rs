use crate::interaction::action::Action;
use godot::classes::Node;
use godot::global::godot_print;
use godot::obj::Base;
use godot::register::{GodotClass, godot_dyn};

#[derive(GodotClass)]
#[class(init, base=Node)]
struct TestAction {
    base: Base<Node>,
}

#[godot_dyn]
impl Action for TestAction {
    fn interact(&mut self) {
        godot_print!("Hello!");
    }
}
