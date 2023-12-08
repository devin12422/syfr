pub mod prelude {
    // Bevy + Bevy_Lunex
    pub use bevy::prelude::*;
    // pub use bevy_lunex::prelude::*;

    // STD + Usefull stuff
    // pub use bevy::window::PrimaryWindow;
    // pub use std::borrow::Borrow;

    // Global access to this data
    // pub use crate::interface::*;
    // pub use crate::MenuAssetCache;
    // pub use crate::MyData;
    // pub use crate::{COLOR_PRIMARY, COLOR_SECONDARY};
}
use bevy_rapier3d::prelude::*;
use camera::PlayerCameraPlugin;
use prelude::*;

mod backend;
mod camera;
mod input;
mod player;
mod utilities;
mod window;
mod world;
use backend::*;
// use gui::GUIPlugin;
use player::PlayerPlugin;
use world::WorldPlugin;
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
    Pause,
}
fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            DefaultPlugins,
            PlayerCameraPlugin,
            // GUIPlugin,
            // PlayerPlugin,
            WorldPlugin,
            BackendPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}
