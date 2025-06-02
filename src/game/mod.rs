use bevy::math::I16Vec2;
use bevy::math::U16Vec2;
use bevy::prelude::*;

mod board;
mod piece;

pub type Coords = U16Vec2;
pub type PieceCoords = I16Vec2;

pub fn plugin(app: &mut App) {}
