use amethyst::ecs::prelude::{Component, DenseVecStorage};



pub struct Mover;

impl Default for Mover {
    fn default() -> Self {
        Mover{}
    }
}

impl Component for Mover {
    type Storage = DenseVecStorage<Self>;
}