use crate::prelude::*;
pub struct PlayerCameraPlugin;
impl Plugin for PlayerCameraPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player_camera);
    }
}
#[derive(Component)]
pub struct PlayerCameraMarker;

fn spawn_player_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 0.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        },
        PlayerCameraMarker,
    ));
}
