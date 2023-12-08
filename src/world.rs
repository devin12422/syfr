use crate::player::*;
use crate::prelude::*;
use crate::AppState;
use bevy_rapier3d::prelude::*;

pub struct WorldPlugin;
impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), (spawn_light, spawn_floor))
            .add_plugins(PlayerPlugin);
    }
}
fn spawn_light(mut commands: Commands) {
    let light = PointLightBundle {
        point_light: PointLight {
            intensity: 2000.0,
            ..Default::default()
        },
        transform: Transform::from_xyz(0.0, 5.0, 0.0),
        ..Default::default()
    };
    commands.spawn(light);
}
fn spawn_floor(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
        .spawn(Collider::cuboid(100.0, 0.1, 100.0))
        .insert(TransformBundle::from(Transform::from_xyz(0.0, -2.0, 0.0)));
    let floor = PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(100.0))),
        material: materials.add(Color::DARK_GREEN.into()),
        ..default()
    };
    commands.spawn(floor);
}
