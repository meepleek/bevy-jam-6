use bevy::platform::collections::HashSet;
use bevy::prelude::*;
use tiny_bail::or_continue;

use super::PieceCoords;
use super::board::TILE_SIZE;
use crate::game::drag::Draggable;
use crate::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, on_piece_added);
}

#[derive(Component, Debug, Clone, PartialEq)]
#[require(TilePieces, Transform, Visibility)]
pub enum Piece {
    Pattern {
        size: u16,
        explosion_tiles: HashSet<PieceCoords>,
    },
    Direction(Dir2),
}

#[derive(Component)]
#[relationship(relationship_target = TilePieces)]
pub struct TilePieceOf(Entity);

#[derive(Component, Default)]
#[relationship_target(relationship = TilePieceOf, linked_spawn)]
pub struct TilePieces(Vec<Entity>);

impl Piece {
    pub fn dir(dir: Dir2) -> Self {
        Self::Direction(dir)
    }

    pub fn pattern<C: Into<PieceCoords>>(explosion_tiles: impl IntoIterator<Item = C>) -> Self {
        let mut explosion_tiles: HashSet<PieceCoords> =
            explosion_tiles.into_iter().map(Into::into).collect();
        if explosion_tiles.is_empty() {
            panic!("Invalid pattern: a pattern piece has to have at least 1 exploing tile");
        }
        explosion_tiles.insert(PieceCoords::ZERO);
        let half_size = explosion_tiles
            .iter()
            .map(|tile| tile.abs().max_element())
            .max_by(|a, b| a.cmp(b))
            .unwrap();
        Self::Pattern {
            size: (half_size * 2 + 1) as u16,
            explosion_tiles,
        }
    }

    pub fn size(&self) -> u16 {
        match self {
            Piece::Pattern { size, .. } => *size,
            Piece::Direction(_) => 1,
        }
    }

    pub fn explosion_tiles(&self) -> Vec<PieceCoords> {
        match self {
            Piece::Pattern {
                explosion_tiles, ..
            } => explosion_tiles.iter().cloned().collect(),
            Piece::Direction(_) => vec![PieceCoords::ZERO],
        }
    }

    pub fn draw_piece_tile(&self, piece_tile: impl Into<PieceCoords>) -> Option<char> {
        let piece_tile = piece_tile.into();
        match self {
            Piece::Pattern {
                explosion_tiles: tiles,
                ..
            } => {
                if piece_tile == PieceCoords::ZERO {
                    return Some('x');
                }
                if tiles.contains(&piece_tile) {
                    Some('*')
                } else {
                    None
                }
            },
            Piece::Direction(dir) => {
                if piece_tile != PieceCoords::ZERO {
                    return None;
                }
                Some(match *dir {
                    Dir2::NORTH => '↑',
                    Dir2::NORTH_EAST => '↗',
                    Dir2::EAST => '→',
                    Dir2::SOUTH_EAST => '↘',
                    Dir2::SOUTH => '↓',
                    Dir2::SOUTH_WEST => '↙',
                    Dir2::WEST => '←',
                    Dir2::NORTH_WEST => '↖',
                    _ => panic!("Invalid direction"),
                })
            },
        }
    }

    fn draw_piece(&self) -> String {
        match self {
            Piece::Pattern { size, .. } => {
                let half_size = *size as i16 / 2;
                (-half_size..=half_size)
                    .map(|y| {
                        (-half_size..=half_size)
                            .map(move |x| self.draw_piece_tile((x, y)).unwrap_or('.'))
                            .collect()
                    })
                    .collect::<Vec<String>>()
                    .join("\n")
            },
            Piece::Direction(_) => self
                .draw_piece_tile(PieceCoords::ZERO)
                .expect("Valid direction piece")
                .to_string(),
        }
    }
}

impl Piece {
    pub fn line() -> Self {
        Self::pattern([(-1, 0), (1, 0)])
    }

    pub fn line_diag() -> Self {
        Self::pattern([(-1, -1), (1, 1)])
    }

    pub fn cross() -> Self {
        Self::pattern([(0, 1), (1, 0), (0, -1), (-1, 0)])
    }

    pub fn cross_diag() -> Self {
        Self::pattern([(1, 1), (1, -1), (-1, -1), (-1, 1)])
    }
}

#[cfg_attr(feature = "native_dev", hot)]
fn on_piece_added(
    mut cmd: Commands,
    added_piece_q: Query<(Entity, &Piece), Added<Piece>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (e, piece) in added_piece_q {
        let mut e_cmd = or_continue!(cmd.get_entity(e));
        e_cmd.insert(Draggable);

        e_cmd.with_children(|b| {
            for tile in piece.explosion_tiles() {
                b.spawn((
                    Mesh2d(meshes.add(Rectangle::new(
                        TILE_SIZE as f32 * 0.9,
                        TILE_SIZE as f32 * 0.9,
                    ))),
                    MeshMaterial2d(materials.add(Color::from(AMBER_300))),
                    Transform::from_xyz(
                        tile.x as f32 * TILE_SIZE as f32,
                        tile.y as f32 * TILE_SIZE as f32,
                        0.0,
                    ),
                    TilePieceOf(b.target_entity()),
                ));
            }
        });
    }
}

#[cfg(test)]
mod tests {
    use pretty_assertions::assert_eq;
    use test_case::test_case;

    use super::*;

    #[test]
    #[should_panic]
    fn invalid_pattern_piece_panics() {
        let _ = Piece::pattern(Vec::<PieceCoords>::new());
    }

    #[test_case([(-1, 0), (1, 0)], 3)]
    #[test_case([(-1, -1), (1, 1)], 3)]
    #[test_case([(0, 2)], 5)]
    fn pattern_piece(explosion_tiles: impl IntoIterator<Item = (i16, i16)>, expected_size: u16) {
        let explosion_tiles: HashSet<PieceCoords> =
            explosion_tiles.into_iter().map(Into::into).collect();
        let piece = Piece::pattern(explosion_tiles.clone());

        let mut expected_explosion_tiles = explosion_tiles.clone();
        // add center tile
        expected_explosion_tiles.insert(PieceCoords::ZERO);

        if let Piece::Pattern {
            size,
            explosion_tiles,
            ..
        } = piece
        {
            assert_eq!(expected_size, size);
            assert_eq!(expected_explosion_tiles, explosion_tiles);
        } else {
            panic!("Invalid piece kind");
        }
    }

    #[test_case(Piece::dir(Dir2::NORTH), 0, 1, None)]
    #[test_case(Piece::dir(Dir2::NORTH), 1, 1, None)]
    #[test_case(Piece::dir(Dir2::NORTH), 0, 0, Some('↑'))]
    #[test_case(Piece::line(), 0, 0, Some('x'))]
    #[test_case(Piece::line(), -1, 0, Some('*'))]
    #[test_case(Piece::line(), 0, 1, None)]
    fn draw_piece_tile(piece: Piece, x: i16, y: i16, expected: Option<char>) {
        let res = piece.draw_piece_tile((x, y));
        assert_eq!(expected, res)
    }

    #[test_case(Piece::dir(Dir2::NORTH), "↑")]
    #[test_case(
        Piece::line(),
        "
...
*x*
...
"
    )]
    #[test_case(
        Piece::line_diag(),
        "
*..
.x.
..*
"
    )]
    #[test_case(
        Piece::cross(),
        "
.*.
*x*
.*.
"
    )]
    #[test_case(
        Piece::cross_diag(),
        "
*.*
.x.
*.*
"
    )]
    fn draw_piece(piece: Piece, expected: &str) {
        let res = piece.draw_piece();
        assert_eq!(expected.trim_ascii(), res)
    }
}
