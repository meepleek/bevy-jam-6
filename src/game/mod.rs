use bevy::math::U16Vec2;
use bevy::prelude::*;

pub mod grid;

pub type Coords = U16Vec2;

pub fn plugin(app: &mut App) {
    app.add_plugins(grid::plugin);
}
