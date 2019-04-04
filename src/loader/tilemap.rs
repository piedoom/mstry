use crate::data::tilemap::TilemapFormat;
use amethyst::{
    assets::{AssetStorage, Handle, Loader},
    ecs::prelude::*,
    renderer::{PngFormat, Texture, TextureMetadata},
};

/// Load an arbitrary texture and return data including the texture handle
///
/// * `world` - Our world
/// * `texture` - the path of the desired texture relative to our assets directory WITHOUT a file extension.
/// All textures are assumed to be PNGs and have RON files of the same name in the same directory.
pub fn load_texture(world: &mut World, path: &str) -> Handle<TilemapFormat> {
    let loader = world.read_resource::<Loader>();

    // get the texture
    let texture_handle = {
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("{}.png", path),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    // Attach loaded bitmap to our tilemap
    loader.load(
        format!("{}.ron", path),
        TilemapFormat,
        texture_handle,
        (),
        &world.read_resource::<AssetStorage<TilemapFormat>>(),
    )
}
