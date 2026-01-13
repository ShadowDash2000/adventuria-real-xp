use crate::interaction::Action;
use godot::classes::Node;
use godot::obj::{Base, Gd};
use godot::prelude::{godot_dyn, GodotClass};

#[derive(GodotClass)]
#[class(init, base=Node)]
struct CreateNode {
    base: Base<Node>,

    #[export]
    node: Option<Gd<Node>>,
}

#[godot_dyn]
impl Action for CreateNode {
    fn interact(&mut self) {
        todo!()
    }
}
