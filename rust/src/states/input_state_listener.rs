use crate::states::input_state::{InputStateManager, InputStates};
use godot::classes::Node;
use godot::obj::{GodotClass, Inherits, WithBaseField};

pub trait InputStateListener: WithBaseField {
    fn on_input_state_changed(&mut self, new_state: InputStates);

    fn connect_to_input_state(&mut self)
    where
        <Self as GodotClass>::Base: Inherits<Node>,
    {
        let input_state = InputStateManager::singleton();
        input_state
            .signals()
            .state_changed()
            .connect_other(self, |this, new_state| {
                this.on_input_state_changed(new_state);
            });
    }
}
