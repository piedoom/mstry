extern crate amethyst;
use crate::state::game::Mstry;

mod data;
mod loader;
mod state;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat, Pipeline, PosNormTex, RenderBundle, Stage},
    utils::application_root_dir,
};

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;

    let assets_path = app_root.join("assets");
    let display_config_path = assets_path.join("display_config.ron");
    let config = DisplayConfig::load(display_config_path);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            .with_pass(DrawFlat::<PosNormTex>::new()),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?;
    let mut game = Application::new(assets_path, Mstry, game_data)?;

    game.run();

    Ok(())
}
