use crate::interaction::interaction::Interaction;
use crate::interaction::interactive::Interactive;
use godot::classes::StaticBody3D;
use godot::obj::{Base, WithBaseField};
use godot::register::{GodotClass, godot_dyn};

#[derive(GodotClass)]
#[class(init, base=StaticBody3D)]
struct InteractiveStaticBody3D {
    base: Base<StaticBody3D>,
}

#[godot_dyn]
impl Interactive for InteractiveStaticBody3D {
    fn interact(&mut self) {
        for child in self.base().get_children().iter_shared() {
            if let Ok(mut interaction) = child.try_dynify::<dyn Interaction>() {
                interaction.dyn_bind_mut().interact();
            }
        }
    }
}
