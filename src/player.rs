// use crate::camera::*;
use bevy::input::mouse::MouseMotion;
use bevy::prelude::*;
use bevy_rapier3d::control::KinematicCharacterController;
use bevy_rapier3d::prelude::*;
pub struct PlayerPlugin;
impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_player)
            .add_systems(Update, player_movement);
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
    mut motion_evr: EventReader<MouseMotion>,
    keys: Res<Input<KeyCode>>,
    time: Res<Time>,
    mut player_q: Query<(&mut Transform, &mut KinematicCharacterController), With<PlayerMarker>>,
) {
    for mut player_transform in player_q.iter_mut() {
        for ev in motion_evr.read() {
            println!("Mouse moved: X: {} px, Y: {} px", ev.delta.x, ev.delta.y);
            // player_transform.rotate_local_axis(, );
            // It may actually be faster to use two rotate functions
            // The gain in using only one functon invocation relies on reduction in sinusoidal function calls
            player_transform.0.rotate_local_y(45.0_f32.to_radians());
        }
        let mut direction: Vec3 = Vec3::ZERO;
        if keys.pressed(KeyCode::W) {
            direction += player_transform.0.forward();
        }
        if keys.pressed(KeyCode::S) {
            direction += player_transform.0.back();
        }
        if keys.pressed(KeyCode::A) {
            direction += player_transform.0.left();
        }
        if keys.pressed(KeyCode::D) {
            direction += player_transform.0.right();
        }
        let movement = direction.normalize_or_zero() * 2.0 * time.delta_seconds();
        player_transform.1.translation = Some(movement);
    }
}

fn spawn_player(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let player = commands
        .spawn((
            LivingBundle {
                pbr: PbrBundle {
                    mesh: meshes.add(Mesh::from(shape::Cube::new(1.0))),
                    material: materials.add(Color::BLUE.into()),
                    transform: Transform::from_xyz(0.0, 0.5, 0.0),
                    ..Default::default()
                },
                // camera:,
                kinematic_controller: KinematicCharacterController::default(),
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
    let camera = commands
        .spawn(Camera3dBundle {
            transform: Transform::from_xyz(0.0, 1.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..Default::default()
        })
        .id();
    commands.entity(player).add_child(camera);
}
