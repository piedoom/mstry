use amethyst::core::{Transform, timing::Time};
use amethyst::ecs::{Join, Read, ReadStorage, System, WriteStorage};
use amethyst::input::InputHandler;
use amethyst::core::math::{Vector3, Unit};
use crate::component::Mover;

const SPEED_MULTIPLIER: f32 = 1.4;

pub struct MoverSystem;

impl<'s> System<'s> for MoverSystem {
  type SystemData = (
    Read<'s, Time>,
    WriteStorage<'s, Transform>,
    ReadStorage<'s, Mover>,
    Read<'s, InputHandler<String, String>>,
  );

  fn run(&mut self, (time, mut transforms, movers, input): Self::SystemData) {
    for (mover, transform) in (&movers, &mut transforms).join() {
      let dir = Unit::try_new(
        Vector3::new(
          input.axis_value("horizontal").unwrap_or(0.0) as f32, 
          input.axis_value("vertical").unwrap_or(0.0) as f32, 
          0.0),
      1.0e-6);
      
      if let Some(d) = dir {
        transform.append_translation_along(d, time.delta_seconds() * 100.0);
        println!("moving {:?}", dir);
      }
    }
  }
}