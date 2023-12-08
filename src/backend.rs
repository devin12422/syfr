use crate::input::*;
use crate::prelude::*;
use crate::window::*;
pub struct BackendPlugin;
impl Plugin for BackendPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((WindowPlugin, InputPlugin));
    }
}
