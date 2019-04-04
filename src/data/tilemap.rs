use amethyst::assets::{
    Asset, Error as AssetsError, ErrorKind as AssetsErrorKind, Handle, ProcessingState,
    Result as AssetsResult, SimpleFormat,
};
use amethyst::ecs::VecStorage;
use amethyst::renderer::{Sprite, Texture, TextureHandle};
use ron::de::from_bytes;
use serde_derive::*;
use std::collections::HashMap;

/// `MapData` is just a `HashMap`. Each string identifier
/// corresponds to the 0-starting index of the atlas
pub type MapData = HashMap<String, usize>;

#[derive(Clone, PartialEq, Eq, Debug, Default, Serialize, Deserialize)]
pub struct TilemapFormat {
    /// Filepath for texture
    pub path: String,
    /// The pixel size of one tile
    pub tile_size: (usize, usize),
    /// The overall size of the texture. Should be divisible by the tile size.
    pub size: (usize, usize),
    pub map: MapData,
}

impl TilemapFormat {
    pub fn tile_grid(&self) -> (usize, usize) {
        (
            self.size.0 / self.tile_size.0,
            self.size.1 / self.tile_size.1,
        )
    }
    // Uses a grid for tiles, not pixel values
    pub fn tile_coordinates_to_index(&self, c: (usize, usize)) -> usize {
        let grid = self.tile_grid();
        c.1 + grid.0 * c.1
    }
    // Uses a grid for tiles, not pixel values
    pub fn index_to_tile_coordinates(&self, i: usize) -> (usize, usize) {
        let grid = self.tile_grid();
        (i % grid.0, i / grid.0)
    }
    pub fn index_to_pixel_coordinates(&self, i: usize) -> (usize, usize) {
        // Get tile coordinates
        let tile_coordinate = self.index_to_tile_coordinates(i);
        // multiply tile coordinates to get pixel values
        (
            tile_coordinate.0 * self.tile_size.0,
            tile_coordinate.1 * self.tile_size.1,
        )
    }
}

impl SimpleFormat<Tilemap> for TilemapFormat {
    const NAME: &'static str = "TILEMAP";

    type Options = Handle<Texture>;

    fn import(&self, bytes: Vec<u8>, texture: Self::Options) -> AssetsResult<Tilemap> {
        // Parse input ron
        let data: TilemapFormat = from_bytes(&bytes).map_err(|_| {
            AssetsError::from_kind(AssetsErrorKind::Format(
                "Failed to parse Ron file for Tilemap",
            ))
        })?;

        // Begin creating all sprites
        let mut sprites: Vec<Sprite> = Vec::with_capacity(self.size.0 * self.size.1);
        self.map.into_iter().map(|(k, v)| {
            let offset = self.index_to_pixel_coordinates(v);
            // Get the associated sprite at index `v`
            sprites[v] = Sprite::from_pixel_values(
                self.size.0 as u32,      // Total width
                self.size.1 as u32,      // Total height
                self.tile_size.0 as u32, // Tile width
                self.tile_size.1 as u32, // Tile height
                offset.0 as u32,         // Offset left
                offset.1 as u32,         // Offset top
                [0.0; 2],                // sp.offsets.unwrap_or([0.0; 2]),
            );
        });

        Ok(Tilemap {
            map: self.map,
            texture: texture,
            sprites: sprites,
        })
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tilemap {
    map: MapData,
    texture: Handle<Texture>,
    sprites: Vec<Sprite>,
}

impl Tilemap {
    /// Return a tile with the string identifier
    pub fn tile(&self, key: &str) -> Option<Sprite> {
        match self.map.get(key) {
            Some(v) => Some(self.sprites[*v]),
            None => None,
        }
    }
}

impl Asset for Tilemap {
    const NAME: &'static str = "mstry::data::tilemap::TilemapFormat";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

pub type TilemapHandle = Handle<Tilemap>;

impl From<Tilemap> for AssetsResult<ProcessingState<Tilemap>> {
    fn from(tilemap: Tilemap) -> AssetsResult<ProcessingState<Tilemap>> {
        Ok(ProcessingState::Loaded(tilemap))
    }
}
