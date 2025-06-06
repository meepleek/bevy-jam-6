use crate::game::tile::TileCoords;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {}

#[derive(Component)]
#[require(TileCoords, Transform, Visibility)]
pub struct Die {}

pub fn die(color: impl Into<Color>, pip_count: u8) -> impl Bundle {
    (
        Die {},
        Sprite::from_color(color.into(), Vec2::splat(50.)),
        children![(
            Text2d::new(pip_count.to_string()),
            TextColor(GRAY_950.into())
        )],
    )
}
