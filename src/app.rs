use bevy::prelude::*;
pub struct AppStatePlugin;
impl Plugin for AppStatePlugin {
    fn build(&self, app: &mut App) {
        app.add_state::<AppState>();
    }
}
#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
enum AppState {
    #[default]
    MainMenu,
    InGame,
}
