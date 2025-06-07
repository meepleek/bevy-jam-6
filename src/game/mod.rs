use bevy::math::I16Vec2;
use bevy::prelude::*;

pub mod action;
pub mod card;
pub mod card_effect;
pub mod die;
pub mod drag;
pub mod grid;
pub mod level;
pub mod player;
pub mod tile;

pub type Coords = I16Vec2;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        tile::plugin,
        drag::plugin,
        grid::plugin,
        card::plugin,
        die::plugin,
        player::plugin,
        level::plugin,
        card_effect::plugin,
        action::plugin,
    ));
}
