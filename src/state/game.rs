use amethyst::core::nalgebra::Vector3;
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection};
pub struct Mstry;

impl SimpleState for Mstry {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        initialize_camera(world);
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_position(Vector3::new(0.0, 0.0, 1.0));
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, 256.0, 0.0, 256.0,
        )))
        .with(transform)
        .build();
}
