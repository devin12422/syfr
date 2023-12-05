use bevy::prelude::*;
mod player;
mod world;
use bevy_rapier3d::prelude::*;

use player::PlayerPlugin;
use world::WorldPlugin;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, WorldPlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}
