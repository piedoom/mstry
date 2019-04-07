extern crate amethyst;
use crate::state::game::Mstry;

mod state;
mod system;
mod component;

use amethyst::{
    core::transform::TransformBundle,
    prelude::*,
    renderer::{DisplayConfig, DrawFlat2D, Pipeline, RenderBundle, Stage},
    utils::application_root_dir,
};

use amethyst::input::InputBundle;


fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());
    let app_root = application_root_dir()?;

    let assets_path = app_root.join("resources");
    let binding_path = assets_path.join("config/bindings.ron");

    let display_config_path = assets_path.join("display_config.ron");
    let config = DisplayConfig::load(display_config_path);
    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.0, 0.0, 0.0, 1.0], 1.0)
            .with_pass(DrawFlat2D::new())
    );

    let input_bundle = InputBundle::<String, String>::new()
        .with_bindings_from_file(binding_path)?;

    let game_data = GameDataBuilder::default()
        .with_bundle(RenderBundle::new(pipe, Some(config)).with_sprite_sheet_processor())?
        .with_bundle(TransformBundle::new())?
        .with_bundle(input_bundle)?
        .with(crate::system::mover::MoverSystem, "mover_system", &["input_system"]);
        
    let mut game = Application::new(assets_path, Mstry, game_data)?;

    game.run();

    Ok(())
}
