use amethyst::assets::AssetStorage;
use amethyst::assets::Loader;
use amethyst::core::math::Vector3;
use amethyst::core::transform::Transform;
use amethyst::assets::Handle;
use amethyst::prelude::*;
use amethyst::renderer::{ScreenDimensions, Camera, Projection, SpriteSheetFormat, SpriteSheet, TextureMetadata, Texture, PngFormat, SpriteRender};
use crate::component as c;
pub struct Mstry;

impl SimpleState for Mstry {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;
        world.register::<SpriteRender>();
        world.register::<Transform>();
        initialize_camera(world, 4f32);
        initialize_map(world);
    }
}

fn initialize_camera(world: &mut World, zoom: f32) {

    // get the window size
    let (width, height) = {
        let dim = world.read_resource::<ScreenDimensions>();
        (dim.width(), dim.height())
    };
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0, width / zoom, 0.0, height / zoom,
        )))
        .with({
            let mut t = Transform::default();
            t.set_translation_z(1.0);
            t
        })
        .with(c::Mover::default())
        .build();
}

fn load_sprite_sheet(world: &mut World, path: &str) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}.png", path),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("{}.ron", path),
        SpriteSheetFormat,
        texture_handle,
        (),
        &sprite_sheet_store,
    )
}

fn initialize_map(world: &mut World) {
    // load the environment map
    let sprites = load_sprite_sheet(world, "textures/environment");

    let sprite_render = SpriteRender {
        sprite_sheet: sprites.clone(),
        sprite_number: 0, // this depends on what texture we want
    };

    
    // set up map tile entities
    world
        .create_entity()
        .with(sprite_render.clone())
        .with({
            let mut t = Transform::default();
            t.set_translation_z(0.0);
            t
        })
        .build();
}