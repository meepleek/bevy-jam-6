use bevy::math::U16Vec2;
use bevy::platform::collections::HashMap;
use bevy::prelude::*;

use super::Coords;

pub const TILE_SIZE: u16 = 64;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, track_position);
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Piece {
    Cross,
    Circle,
    Triangle,
    // Square?
}

impl Piece {
    pub fn char(&self) -> char {
        match &self {
            Piece::Cross => 'x',
            Piece::Circle => 'o',
            Piece::Triangle => '▲',
        }
    }
}

#[derive(Component, Debug, PartialEq)]
#[require(Transform)]
pub struct Grid {
    width: u16,
    heigth: u16,
    tile_size: u16,
    center_global_position: Vec2,
    tiles: HashMap<Coords, Piece>,
}

#[derive(Debug, PartialEq)]
pub enum PlaceError {
    Taken,
    OutOfBounds,
}

impl Grid {
    pub fn new(width: u16, heigth: u16, tile_size: u16) -> Self {
        if width == 0 || heigth == 0 {
            panic!("Invalid dimension - no dimension can be 0");
        }
        Self {
            width,
            heigth,
            tile_size,
            tiles: HashMap::default(),
            center_global_position: Vec2::ZERO,
        }
    }

    pub fn world_center(&self) -> Vec2 {
        self.center_global_position
    }

    pub fn tile_size(&self) -> u16 {
        self.tile_size
    }

    pub fn grid_size(&self) -> U16Vec2 {
        (self.width, self.heigth).into()
    }

    pub fn size(&self) -> Vec2 {
        self.grid_size().as_vec2() * TILE_SIZE as f32
    }

    pub fn world_to_tile(&self, pos: Vec2) -> Option<Coords> {
        // transform world position to board space (like screen space but in tiles)
        let half_size = self.size() / 2.;
        let x = half_size.x - self.center_global_position.x + pos.x;
        let y = half_size.y + self.center_global_position.y - pos.y;
        let pos_on_board = Vec2::new(x, y);
        let coords = (pos_on_board / TILE_SIZE as f32).floor().as_i16vec2();
        if coords.min_element() < 0
            || coords.x >= self.width as i16
            || coords.y >= self.heigth as i16
        {
            return None;
        }

        Some(coords.as_u16vec2())
    }

    pub fn tile_to_world(&self, tile: Coords) -> Option<Vec2> {
        if tile.x >= self.width || tile.y >= self.heigth {
            return None;
        }

        let half_size = self.size() / 2.;
        let half_tile = TILE_SIZE as f32 / 2.;
        let tile_world = tile.as_vec2() * TILE_SIZE as f32;
        let x = tile_world.x + self.center_global_position.x + half_tile - half_size.x;
        let y = -tile_world.y + self.center_global_position.y - half_tile + half_size.y;
        Some(Vec2::new(x, y))
    }

    pub fn can_place_at(&self, coords: Coords) -> Result<(), PlaceError> {
        if coords.x >= self.width || coords.y >= self.heigth {
            return Err(PlaceError::OutOfBounds);
        } else if self.tiles.contains_key(&coords) {
            return Err(PlaceError::Taken);
        }

        Ok(())
    }

    pub fn place_piece(&mut self, piece: Piece, coords: Coords) -> Result<(), PlaceError> {
        self.can_place_at(coords)?;
        self.tiles.insert(coords, piece);

        Ok(())
    }
}

fn track_position(mut board_q: Query<(&mut Grid, &GlobalTransform), Changed<GlobalTransform>>) {
    for (mut board, t) in &mut board_q {
        board.center_global_position = t.translation().truncate();
    }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;
    use tracing_test::traced_test;

    use super::*;

    #[test_case(0., 0., 0., 0. => Some(Coords::ONE))]
    #[test_case(64.,-64., 0., 0. => Some(Coords::ZERO))]
    #[test_case(64.,-64., 20., -20. => Some(Coords::ZERO))]
    #[test_case(64.,-64., 40., -40. => Some(Coords::ONE))]
    #[test_case(64., -64., 64., 0. => Some(Coords::new(1, 0)))]
    #[test_case(0., 0., 120., 0. => None)]
    #[test_case(0., 0., -128., 0. => None)]
    #[test_case(0., 0., 0., 120. => None)]
    #[test_case(0., 0., 0., -128. => None)]
    #[traced_test]
    fn world_to_tile(map_x: f32, map_y: f32, world_x: f32, world_y: f32) -> Option<Coords> {
        let mut board = Grid::new(3, 3, 64);
        board.center_global_position = Vec2::new(map_x, map_y);

        board.world_to_tile(Vec2::new(world_x, world_y))
    }

    #[test_case(0., 0., 0, 0 => Some(Vec2::new(-64., 64.)))]
    #[test_case(0., 0., 1, 1 => Some(Vec2::new(0., 0.)))]
    // todo: fix failing test
    // #[test_case(64.,-64., 0, 0 => Some(Vec2::new(64., -64.)))]
    #[test_case(64.,-64., 2, 2 => Some(Vec2::new(128., -128.)))]
    #[test_case(0.,0., 3, 0 => None)]
    #[test_case(0.,0., 0, 3 => None)]
    #[traced_test]
    fn tile_to_world(map_x: f32, map_y: f32, tile_x: u16, tile_y: u16) -> Option<Vec2> {
        let mut board = Grid::new(3, 3, 64);
        board.center_global_position = Vec2::new(map_x, map_y);

        board.tile_to_world(Coords::new(tile_x, tile_y))
    }

    #[test_case(0, 0 => matches Ok(_))]
    #[test_case(3, 3 => matches Ok(_))]
    #[test_case(4, 6 => matches Ok(_))]
    #[test_case(6, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 9 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(50, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 50 => matches Err(PlaceError::OutOfBounds))]
    fn can_place_at_coords(x: u16, y: u16) -> Result<(), PlaceError> {
        let board = Grid::new(6, 9, 32);
        board.can_place_at((x, y).into())
    }

    #[test_case(
        3,
        Coords::ZERO,
        Coords::new(1, 0),
        "
xo.
...
...
"
    )]
    #[test_case(
        3,
        Coords::new(1, 0),
        Coords::new(1, 2),
        "
.x.
...
.o.
"
    )]

    fn grid(board_size: u16, coords_1: Coords, coords_2: Coords, expected: &str) {
        let mut board = Grid::new(board_size, board_size, 32);
        board
            .place_piece(Piece::Cross, coords_1)
            .expect("placed 1st piece");
        board
            .place_piece(Piece::Circle, coords_2)
            .expect("placed 2nd piece");
        let mut explosion_debug_grid = String::default();

        for y in 0..board.heigth {
            for x in 0..board.width {
                let tile = Coords::new(x, y);
                if let Some(piece) = board.tiles.get(&tile) {
                    explosion_debug_grid.push(piece.char());
                } else {
                    explosion_debug_grid.push('.');
                }
            }

            if y < board_size - 1 {
                explosion_debug_grid.push('\n');
            }
        }

        pretty_assertions::assert_eq!(expected.trim_ascii(), explosion_debug_grid);
    }

    #[test]
    fn cannot_place_at_coords_when_taken() {
        let coords: Coords = (3, 3).into();
        let mut board = Grid::new(6, 6, 32);
        board
            .place_piece(Piece::Cross, coords)
            .expect("Place first piece");

        assert_eq!(board.can_place_at(coords), Err(PlaceError::Taken));
    }
}
