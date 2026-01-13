use crate::interaction::Action;
use godot::classes::Node;
use godot::global::godot_print;
use godot::obj::Base;
use godot::register::{godot_dyn, GodotClass};

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
