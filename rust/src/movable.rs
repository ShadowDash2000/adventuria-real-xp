use godot::builtin::real;
use godot::classes::CharacterBody3D;
use godot::obj::Gd;
use godot::prelude::Vector3;

pub trait Movable3D {
    fn get_speed(&self) -> real;
    fn get_acceleration(&self) -> f64;
    fn get_jump_impulse(&self) -> real;
    fn get_gravity(&self) -> real;

    fn move_and_slide(&self, mut base: Gd<CharacterBody3D>, direction: Vector3, delta: f64) {
        let mut velocity = base.get_velocity();

        velocity = self.apply_horizontal_movement(velocity, direction, delta);
        velocity = self.apply_vertical_movement(base.clone(), velocity, delta);

        base.set_velocity(velocity);
        base.move_and_slide();
    }

    fn apply_horizontal_movement(
        &self,
        velocity: Vector3,
        direction: Vector3,
        delta: f64,
    ) -> Vector3 {
        let speed = self.get_speed();
        let acceleration = self.get_acceleration();

        let target_horizontal_velocity = if let Some(dir) = direction.try_normalized() {
            dir * speed
        } else {
            Vector3::ZERO
        };

        let mut horizontal_velocity = Vector3::new(velocity.x, 0.0, velocity.z);
        horizontal_velocity = horizontal_velocity
            .move_toward(target_horizontal_velocity, (acceleration * delta) as real);

        Vector3::new(horizontal_velocity.x, velocity.y, horizontal_velocity.z)
    }

    fn apply_vertical_movement(
        &self,
        base: Gd<CharacterBody3D>,
        velocity: Vector3,
        delta: f64,
    ) -> Vector3 {
        let y = if base.is_on_floor() && velocity.y <= 0.0 {
            0.0
        } else {
            velocity.y + self.get_gravity() * delta as real
        };

        Vector3::new(velocity.x, y, velocity.z)
    }

    fn jump(&self, mut base: Gd<CharacterBody3D>) {
        if base.is_on_floor() {
            let velocity = base.get_velocity();

            base.set_velocity(Vector3::new(
                velocity.x,
                velocity.y + self.get_jump_impulse(),
                velocity.z,
            ));
        }
    }
}
