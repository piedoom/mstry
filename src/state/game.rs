use crate::loader::tilemap::load_tilemap;
use amethyst::assets::AssetStorage;
use amethyst::assets::Loader;
use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::prelude::*;
use amethyst::renderer::{Camera, Projection};
pub struct Mstry;

impl SimpleState for Mstry {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        initialize_camera(world);

        world.add_resource(AssetStorage::<crate::data::tilemap::Tilemap>::new());

        let texture_handle = load_tilemap(world, "/textures/environment");
        world
            .create_entity()
            .with(Transform::default())
            .with(texture_handle.clone())
            .build();
    }
}

fn initialize_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_translation(Vector3::new(0.0, 0.0, 1.0));
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, 256.0, 0.0, 256.0,
        )))
        .with(transform.clone())
        .build();
}
