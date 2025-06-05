use bevy::math::I16Vec2;
use bevy::math::U16Vec2;
use bevy::prelude::*;

pub mod board;
pub mod card;
pub mod drag;
pub mod tile;

pub type Coords = U16Vec2;
pub type PieceCoords = I16Vec2;

pub fn plugin(app: &mut App) {
    app.add_plugins((tile::plugin, drag::plugin, board::plugin, card::plugin));
}
