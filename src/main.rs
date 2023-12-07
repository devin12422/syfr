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

mod app;
mod camera;
mod gui;
mod player;
mod utilities;
mod window;
mod world;

use app::*;
use gui::GUIPlugin;
use player::PlayerPlugin;
use window::WindowPlugin;
use world::WorldPlugin;
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins,
            AppStatePlugin,
            PlayerCameraPlugin,
            GUIPlugin,
            PlayerPlugin,
            WorldPlugin,
            WindowPlugin,
        ))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::default())
        .run();
}
