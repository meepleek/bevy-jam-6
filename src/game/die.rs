use crate::game::tile::TileCoords;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {}

#[derive(Component, Default)]
pub enum Die {
    D2,
    D4,
    #[default]
    D6,
    D8,
    D12,
    D20,
}

#[derive(Component, Deref, DerefMut)]
#[require(TileCoords, Die, Transform, Visibility)]
pub struct Pips(pub u8);

pub fn die(color: impl Into<Color>, pip_count: u8) -> impl Bundle {
    (
        Pips(5),
        Sprite::from_color(color.into(), Vec2::splat(50.)),
        children![(
            Text2d::new(pip_count.to_string()),
            TextColor(GRAY_950.into())
        )],
    )
}
