use bevy::prelude::*;
use bevy::window::*;
pub struct WindowPlugin;
impl Plugin for WindowPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(PostStartup, initialize_resources)
            .add_systems(Update, (handle_cursor));
    }
}
#[derive(Resource)]
pub struct WindowInfoResource {
    pub resolution: Vec2,
}

fn initialize_resources(
    mut commands: Commands,
    mut window_q: Query<&mut Window, (With<Window>, With<PrimaryWindow>)>,
) {
    window_q.single_mut().cursor.grab_mode = CursorGrabMode::Confined;
    commands.insert_resource(WindowInfoResource {
        resolution: Vec2 {
            x: window_q.single().resolution.physical_width() as f32,
            y: window_q.single().resolution.physical_height() as f32,
        },
    });
}
// fn on_window_resize(
//     mut ev_resize: EventReader<WindowResized>,
//     mut window_info: ResMut<WindowInfoResource>,
// ) {
//     for ev in ev_resize.read() {
//         window_info.resolution.x = ev.width;
//         window_info.resolution.y = ev.height;
//     }
// }
fn handle_cursor(
    keys: Res<Input<KeyCode>>,
    mut ev_focus: EventReader<WindowFocused>,
    mut window_q: Query<&mut Window, (With<Window>, With<PrimaryWindow>)>,
) {
    if keys.pressed(KeyCode::Escape) {
        for mut window in window_q.iter_mut() {
            window.cursor.grab_mode = CursorGrabMode::None;
        }
    }
    for ev in ev_focus.read() {
        window_q.get_mut(ev.window).unwrap().cursor.grab_mode = if ev.focused {
            CursorGrabMode::Confined
        } else {
            CursorGrabMode::None
        };
    }
}
