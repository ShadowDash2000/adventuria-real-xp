use godot::classes::InputEvent;
use godot::obj::Gd;

pub trait State {
    fn enter(&self);

    fn exit(&self);

    fn physics_process(&mut self, delta: f64);

    fn input(&mut self, event: Gd<InputEvent>);
}
