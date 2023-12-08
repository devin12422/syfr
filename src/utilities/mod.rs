use bevy::app::AppLabel;

use crate::prelude::*;
pub mod constants;
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum MyLabel {
    /// everything that handles input
    Input,
    /// everything that updates player state
    Player,
    /// everything that moves things (works with transforms)
    Movement,
    /// systems that update the world map
    Map,
}
