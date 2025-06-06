use bevy::math::U16Vec2;
use bevy::prelude::*;

pub mod card;
pub mod die;
pub mod drag;
pub mod grid;
pub mod player;
pub mod tile;

pub type Coords = U16Vec2;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        tile::plugin,
        drag::plugin,
        grid::plugin,
        card::plugin,
        die::plugin,
        player::plugin,
    ));
}
