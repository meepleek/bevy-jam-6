use bevy::color::palettes::css::BLACK;

use crate::game::tile::TileCoords;
use crate::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, update_die_face);
}

relationship_1_to_1!(DieFace, DieFaceRoot);

#[derive(Default, Debug)]
pub enum DieKind {
    D2,
    D4,
    #[default]
    D6,
    D8,
    D12,
    D20,
}
impl DieKind {
    pub fn max_pips(&self) -> u8 {
        match self {
            DieKind::D2 => 2,
            DieKind::D4 => 4,
            DieKind::D6 => 6,
            DieKind::D8 => 8,
            DieKind::D12 => 12,
            DieKind::D20 => 20,
        }
    }

    pub fn show_pips(&self) -> bool {
        matches!(self, DieKind::D2 | DieKind::D4 | DieKind::D6)
    }
}

#[derive(Component, Debug)]
#[require(TileCoords, Transform, Visibility)]
pub struct Die {
    pub pip_count: u8,
    pub kind: DieKind,
}
impl Die {
    pub fn pip_positions(&self) -> Option<Vec<Vec2>> {
        if self.kind.show_pips() {
            Some(match self.pip_count {
                0 => Vec::default(),
                1 => vec![Vec2::ZERO],
                2 => vec![Vec2::ONE, Vec2::NEG_ONE],
                3 => vec![Vec2::ONE, Vec2::ZERO, Vec2::NEG_ONE],
                4 => vec![
                    Vec2::ONE,
                    Vec2::NEG_ONE,
                    Vec2::new(1., -1.),
                    Vec2::new(-1., 1.),
                ],
                5 => vec![
                    Vec2::ONE,
                    Vec2::NEG_ONE,
                    Vec2::new(1., -1.),
                    Vec2::new(-1., 1.),
                    Vec2::ZERO,
                ],
                6 => vec![
                    Vec2::ONE,
                    Vec2::X,
                    Vec2::new(1., -1.),
                    Vec2::NEG_ONE,
                    Vec2::NEG_X,
                    Vec2::new(-1., 1.),
                ],
                _ => unimplemented!("One simply does not draw this many pips"),
            })
        } else {
            None
        }
    }
}

pub fn die(color: impl Into<Color>, die: Die) -> impl Bundle {
    (
        die,
        Sprite::from_color(color.into(), Vec2::splat(50.)),
        // children![(
        //     Text2d::new(pip_count.to_string()),
        //     TextColor(GRAY_950.into())
        // )],
    )
}

fn update_die_face(
    die_q: Query<(Entity, &Die, Option<&DieFaceRoot>), Changed<Die>>,
    mut cmd: Commands,
) {
    for (e, die, face_root) in &die_q {
        if let Some(face_root) = face_root {
            or_continue!(cmd.get_entity(face_root.entity())).try_despawn();
        }
        or_continue!(cmd.get_entity(e))
            .try_remove::<DieFaceRoot>()
            .with_children(|b| {
                if die.kind.show_pips() {
                    b.spawn((
                        DieFace(b.target_entity()),
                        Visibility::default(),
                        Transform::default(),
                    ))
                    .with_children(|b| {
                        for pos in die.pip_positions().expect("Die with pips") {
                            b.spawn((
                                Sprite::from_color(BLACK, Vec2::splat(10.)),
                                Transform::from_translation((pos * 16.).extend(1.)),
                            ));
                        }
                    });
                } else {
                    b.spawn((
                        Text2d::new(die.pip_count.to_string()),
                        TextColor(GRAY_950.into()),
                        DieFace(b.target_entity()),
                    ));
                }
            });
    }
}
