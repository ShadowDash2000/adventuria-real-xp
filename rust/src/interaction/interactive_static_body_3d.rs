use crate::interaction::{Action, Interactive};
use godot::classes::StaticBody3D;
use godot::obj::{Base, WithBaseField};
use godot::register::{godot_dyn, GodotClass};

#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
struct InteractiveStaticBody3D {
    base: Base<StaticBody3D>,
}

#[godot_dyn]
impl Interactive for InteractiveStaticBody3D {
    fn interact(&mut self) {
        for child in self.base().get_children().iter_shared() {
            if let Ok(mut interaction) = child.try_dynify::<dyn Action>() {
                interaction.dyn_bind_mut().interact();
            }
        }
    }
}
