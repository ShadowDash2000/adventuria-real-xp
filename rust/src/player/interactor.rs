use crate::interaction::Interactive;
use godot::classes::{IRayCast3D, InputEvent, RayCast3D};
use godot::obj::{Base, Gd, WithBaseField};
use godot::register::{godot_api, GodotClass};

#[derive(GodotClass)]
#[class(init, base=RayCast3D)]
struct PlayerInteractor {
    base: Base<RayCast3D>,
}

#[godot_api]
impl IRayCast3D for PlayerInteractor {
    fn input(&mut self, event: Gd<InputEvent>) {
        if event.is_action_pressed("interact") {
            self.try_interact();
        }
    }
}

impl PlayerInteractor {
    fn try_interact(&mut self) {
        self.base_mut().force_raycast_update();

        let Some(collider) = self.base().get_collider() else {
            return;
        };

        if let Ok(mut interaction) = collider.try_dynify::<dyn Interactive>() {
            interaction.dyn_bind_mut().interact();
        }
    }
}
