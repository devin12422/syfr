pub mod prelude {
    // use crate::prelude::*;
    pub use bevy_lunex::prelude::*;
}
use crate::camera::*;
use crate::gui::prelude::*;
use crate::prelude::*;
use crate::utilities::constants::*;
use bevy::window::PrimaryWindow;
// pub mod components;

pub struct GUIPlugin;
impl Plugin for GUIPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(LunexUiPlugin2D::<MyData>::new())
            .add_systems(PreStartup, presetup)
            .add_systems(Startup, setup);
        // .add_systems(PostStartup, initialize_resources);
        // .add_systems(Update, (handle_cursor));
    }
}
#[derive(Resource)]
pub struct MenuAssetCache {
    pub font: Handle<Font>,
}
#[derive(Debug, Clone, Component, Default)]
pub struct MyData {
    pub animate_trigger: bool,
    pub animate_slider: f32,
}
fn presetup(mut commands: Commands, asset_server: Res<AssetServer>) {}
fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    // assets: Res<MenuAssetCache>,
    // mut textures: ResMut<Assets<TextureAtlas>>,
    window: Query<Entity, (With<Window>, With<PrimaryWindow>)>,
) {
    // Start playing the main menu music
    // commands.spawn(
    //     AudioBundle {
    //         source: asset_server.load("sounds/main_menu.ogg"),
    //         settings: PlaybackSettings::LOOP.with_volume(bevy::audio::Volume::new_relative(0.5)),
    //     }
    // );
    // commands.spawn(camera());

    // Spawn cursor
    // commands.spawn((
    //     Cursor::new()
    //         .with_os_cursor(false)
    //         .add_sprite_offset(Vec2::splat(14.0))
    //         .add_sprite_offset(Vec2::new(10.0, 12.0))
    //         .add_sprite_offset(Vec2::splat(40.0)),
    //     SpriteSheetBundle {
    //         texture_atlas: textures.add(TextureAtlas::from_grid(
    //             asset_server.load("images/cursor.png"),
    //             Vec2::splat(80.0),
    //             3,
    //             1,
    //             None,
    //             None,
    //         )),
    //         transform: Transform {
    //             translation: Vec3::new(0.0, 0.0, 800.0),
    //             scale: Vec3::new(0.5, 0.5, 1.0),
    //             ..default()
    //         },
    //         sprite: TextureAtlasSprite {
    //             color: COLOR_SECONDARY.with_a(2.0).with_l(0.68),
    //             anchor: bevy::sprite::Anchor::TopLeft,
    //             ..default()
    //         },
    //         ..default()
    //     },
    // ));

    // Create new UiTree (a UI context / DOM)
    // let mut tree: UiTree<MyData> = UiTree::new("Interface");

    // Construct the route Menu first
    // rt::Menu::construct(&mut commands, &assets, &mut tree).unwrap();

    // Print nice debug tree in console
    // println!("{}", tree.tree());

    // Insert the UI into the window
    // let window = window.single();
    // commands.entity(window).insert(tree.bundle());
}
