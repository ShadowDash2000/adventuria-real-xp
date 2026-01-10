use crate::player::state_controller::{PlayerStateController, PlayerStates};
use godot::classes::Node;
use godot::obj::{GodotClass, Inherits, WithBaseField};

pub trait PlayerState: WithBaseField {
    fn on_player_state_changed(&mut self, new_state: PlayerStates);

    fn connect_to_player_state(&mut self)
    where
        <Self as GodotClass>::Base: Inherits<Node>,
    {
        let node = self.base().clone().upcast::<Node>();

        if let Some(player_state) = node
            .get_parent()
            .and_then(|p| p.try_cast::<PlayerStateController>().ok())
        {
            player_state
                .signals()
                .state_changed()
                .connect_other(self, |this, new_state| {
                    this.on_player_state_changed(new_state);
                });
        }
    }
}
