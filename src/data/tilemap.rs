use amethyst::assets::{Asset, Handle, ProcessingState, SimpleFormat};
use amethyst::ecs::{Component, VecStorage};
use amethyst::error::Error;
use amethyst::renderer::{Sprite, Texture};
use ron::de::from_bytes;
use serde_derive::*;
use std::collections::HashMap;

/// `MapData` is just a `HashMap`. Each string identifier
/// corresponds to the 0-starting index of the atlas
pub type MapData = HashMap<String, usize>;

#[derive(Clone, Debug, Default, Serialize, Deserialize)]
struct TilemapData {
    /// The pixel size of one tile
    pub tile_size: (usize, usize),
    /// The overall size of the texture. Should be divisible by the tile size.
    pub size: (usize, usize),
    pub mapping: MapData,
}

impl TilemapData {
    fn tile_grid(&self) -> (usize, usize) {
        (
            self.size.0 / self.tile_size.0,
            self.size.1 / self.tile_size.1,
        )
    }
    // Uses a grid for tiles, not pixel values
    fn tile_coordinates_to_index(&self, c: (usize, usize)) -> usize {
        let grid = self.tile_grid();
        c.1 + grid.0 * c.1
    }
    // Uses a grid for tiles, not pixel values
    fn index_to_tile_coordinates(&self, i: usize) -> (usize, usize) {
        let grid = self.tile_grid();
        (i % grid.0, i / grid.0)
    }
    fn index_to_pixel_coordinates(&self, i: usize) -> (usize, usize) {
        // Get tile coordinates
        let tile_coordinate = self.index_to_tile_coordinates(i);
        // multiply tile coordinates to get pixel values
        (
            tile_coordinate.0 * self.tile_size.0,
            tile_coordinate.1 * self.tile_size.1,
        )
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TilemapFormat;

impl From<Tilemap> for Result<ProcessingState<Tilemap>, Error> {
    fn from(tilemap: Tilemap) -> Result<ProcessingState<Tilemap>, Error> {
        Ok(ProcessingState::Loaded(tilemap))
    }
}

impl SimpleFormat<Tilemap> for TilemapFormat {
    const NAME: &'static str = "TILEMAP";

    type Options = Handle<Texture>;

    fn import(&self, bytes: Vec<u8>, texture: Self::Options) -> Result<Tilemap, Error> {
        // Parse input ron
        let data: TilemapData = from_bytes(&bytes)?;

        // Begin creating all sprites
        let mut sprites: Vec<Sprite> = Vec::with_capacity(data.size.0 * data.size.1);
        let _ = data.clone().mapping.into_iter().map(|(_, v)| {
            let offset = data.index_to_pixel_coordinates(v);
            // Get the associated sprite at index `v`
            sprites[v] = Sprite::from_pixel_values(
                data.size.0 as u32,      // Total width
                data.size.1 as u32,      // Total height
                data.tile_size.0 as u32, // Tile width
                data.tile_size.1 as u32, // Tile height
                offset.0 as u32,         // Offset left
                offset.1 as u32,         // Offset top
                [0.0; 2],                // sp.offsets.unwrap_or([0.0; 2]),
            );
        });

        Ok(Tilemap {
            mapping: data.mapping,
            texture: texture,
            sprites: sprites,
        })
    }
}

impl Asset for TilemapFormat {
    const NAME: &'static str = "mstry::data::tilemap::TilemapFormat";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

#[derive(Clone, PartialEq, Debug)]
pub struct Tilemap {
    mapping: MapData,
    texture: Handle<Texture>,
    sprites: Vec<Sprite>,
}

impl Tilemap {
    /// Return a tile with the string identifier
    pub fn tile(&self, key: &str) -> Option<&Sprite> {
        match self.mapping.get(key) {
            Some(v) => Some(&self.sprites[*v]),
            None => None,
        }
    }
}

impl Asset for Tilemap {
    const NAME: &'static str = "mstry::data::tilemap::Tilemap";
    type Data = Self;
    type HandleStorage = VecStorage<Handle<Self>>;
}

pub type TilemapHandle = Handle<Tilemap>;

#[derive(Clone, Debug, PartialEq)]
pub struct TilemapSpriteRender {
    /// Handle to the sprite sheet of the sprite
    pub sprite_sheet: TilemapHandle,
    /// Index of the sprite on the sprite sheet
    pub sprite_number: usize,
}

impl Component for TilemapSpriteRender {
    type Storage = VecStorage<Self>;
}
