use bevy::prelude::*;
mod player;
mod window;
mod world;
use bevy_rapier3d::prelude::*;

use player::PlayerPlugin;
use window::WindowPlugin;
use world::WorldPlugin;
fn main() {
    App::new()
        .add_plugins((DefaultPlugins, PlayerPlugin, WorldPlugin, WindowPlugin))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}
