// use crate::camera::*;
use crate::camera::*;
use crate::prelude::*;
use crate::AppState;
use bevy::input::mouse::MouseMotion;
use bevy_rapier3d::control::KinematicCharacterController;
use bevy_rapier3d::prelude::*;
// use bevy_rapier3d::rapier::crossbeam::epoch::Pointable;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::InGame), spawn_player)
            .add_systems(Update, player_movement.run_if(in_state(AppState::InGame)));
    }
}
#[derive(Component)]
struct PlayerMarker;
#[derive(Bundle)]
struct LivingBundle {
    pbr: PbrBundle,
    kinematic_controller: KinematicCharacterController,
    collider: Collider,
    rigidbody: RigidBody,
    // marker: PlayerMarker,
}

fn player_movement(
    mut commands: Commands,
    mut motion_evr: EventReader<MouseMotion>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    window_info: Res<crate::window::WindowInfoResource>,
    mut transform_set: ParamSet<(
        Query<
            (
                &mut Transform,
                &mut KinematicCharacterController,
                // Option<&mut Parent>,
            ),
            With<PlayerMarker>,
        >,
        Query<&mut Transform, With<PlayerCameraMarker>>,
    )>,
    player_parent_q: Query<&GlobalTransform>,
) {
    for ev in motion_evr.read() {
        // player_transform.rotate_local_axis(, );
        // It may actually be faster to use two rotate functions
        // The gain in using only one functon invocation relies on reduction in sinusoidal function calls
        // (player_transform.2)
        transform_set
            .p1()
            .single_mut()
            .rotate_local_x(-1.00 * 45.0_f32.to_radians() * ev.delta.y / window_info.resolution.y);
        transform_set
            .p0()
            .single_mut()
            .0
            .rotate_local_y(-1.00 * 45.0_f32.to_radians() * ev.delta.x / window_info.resolution.x);
    }
    let mut player_transform = transform_set.p0();
    let mut direction: Vec3 = Vec3::ZERO;
    if keys.pressed(KeyCode::W) {
        direction += player_transform.single_mut().0.forward();
    }
    if keys.pressed(KeyCode::S) {
        direction += player_transform.single_mut().0.back();
    }
    if keys.pressed(KeyCode::A) {
        direction += player_transform.single_mut().0.left();
    }
    if keys.pressed(KeyCode::D) {
        direction += player_transform.single_mut().0.right();
    }
    let movement = direction.normalize_or_zero() * 2.0 * time.delta_seconds();
    player_transform.single_mut().1.translation = Some(movement);
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    camera_q: Query<Entity, With<PlayerCameraMarker>>,
) {
    let player = commands
        .spawn((
            LivingBundle {
                pbr: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Capsule::default())),
                    material: materials.add(Color::BLUE.into()),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..Default::default()
                },
                // camera:,
                kinematic_controller: KinematicCharacterController {
                    snap_to_ground: Some(CharacterLength::Absolute(0.5)),
                    ..default()
                },
                collider: Collider::ball(0.5),
                rigidbody: RigidBody::KinematicPositionBased,
            },
            PlayerMarker,
        ))
        .id();
    // commands
    //     .spawn(RigidBody::Dynamic)
    //     .insert(Collider::ball(0.5))
    //     .insert(Restitution::coefficient(0.7))
    //     .insert(TransformBundle::from(Transform::from_xyz(0.0, 4.0, 0.0)));
    commands.entity(player).add_child(camera_q.single());
}
