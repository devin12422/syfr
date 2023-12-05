// mod Movable;
// use Movable::movable;
// #[derive(Bundle)]
// struct Living{};
// #[derive(Bundlce)]
// struct Playc
use bevy::prelude::*;
pub trait MovementHandler {
    fn handle_movement(&self) -> Transform;
}
