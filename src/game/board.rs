use bevy::math::U16Vec2;
use bevy::platform::collections::HashMap;
use bevy::platform::collections::HashSet;
use bevy::prelude::*;

use super::Coords;
use super::piece::Piece;

const DEFAULT_SIZE: u16 = 6;

#[derive(Component, Debug)]
pub struct Board {
    width: u16,
    heigth: u16,
    tiles: HashMap<Coords, Piece>,
    explosion_grid: HashSet<Coords>,
}

impl Default for Board {
    fn default() -> Self {
        Self::new(DEFAULT_SIZE, DEFAULT_SIZE)
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum PlaceError {
    Taken,
    OutOfBounds,
}

impl Board {
    pub fn new(width: u16, heigth: u16) -> Self {
        if width == 0 || heigth == 0 {
            panic!("Invalid dimension - no dimension can be 0");
        }
        Self {
            width,
            heigth,
            tiles: HashMap::default(),
            explosion_grid: HashSet::default(),
        }
    }

    pub fn size(&self) -> U16Vec2 {
        (self.width, self.heigth).into()
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
        for piece_tile in &piece.explosion_tiles() {
            if let (Some(x), Some(y)) = (
                coords.x.checked_add_signed(piece_tile.x),
                coords.y.checked_add_signed(piece_tile.y),
            ) {
                self.explosion_grid.insert((x, y).into());
            }
        }
        self.tiles.insert(coords, piece);

        Ok(())
    }

    // fn tile_index(&self, coords: Coords) -> usize {
    //     usize::from(coords.y * self.width + coords.x)
    // }

    // pub fn clear(&mut self) {
    //     for f in self.fields.iter_mut() {
    //         *f = false;
    //     }
    // }

    // pub fn tile_coords_to_tile_index(&self, coords: UVec2) -> usize {
    //     (coords.y * self.width as u32 + coords.x) as usize
    // }

    // pub fn is_section_empty(&self, section_index: usize) -> bool {
    //     self.get_section_by_section_index(section_index)
    //         .iter()
    //         .all(|i| !self.fields[*i])
    // }
}

#[cfg(test)]
mod tests {
    use test_case::test_case;

    use super::*;

    #[test_case(0, 0 => matches Ok(_))]
    #[test_case(3, 3 => matches Ok(_))]
    #[test_case(4, 6 => matches Ok(_))]
    #[test_case(6, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 9 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(50, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 50 => matches Err(PlaceError::OutOfBounds))]
    fn can_place_at_coords(x: u16, y: u16) -> Result<(), PlaceError> {
        let board = Board::new(6, 9);
        board.can_place_at((x, y).into())
    }

    #[test_case(
        3,
        Piece::cross(),
        Coords::ZERO,
        Coords::new(1, 0),
        "
xxx
xx.
...
"
    )]
    #[test_case(
        3,
        Piece::line(),
        Coords::new(1, 0),
        Coords::new(1, 2),
        "
xxx
...
xxx
"
    )]

    fn explosion_grid(
        board_size: u16,
        piece: Piece,
        coords_1: Coords,
        coords_2: Coords,
        expected: &str,
    ) {
        let mut board = Board::new(board_size, board_size);
        board
            .place_piece(piece.clone(), coords_1)
            .expect("placed 1st piece");
        board
            .place_piece(piece, coords_2)
            .expect("placed 2nd piece");
        let mut explosion_debug_grid = String::default();

        for y in 0..board.heigth {
            for x in 0..board.width {
                let tile = Coords::new(x, y);
                explosion_debug_grid.push(if board.explosion_grid.contains(&tile) {
                    'x'
                } else {
                    '.'
                });
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
        let mut board = Board::new(6, 6);
        board
            .place_piece(Piece::Direction(Dir2::NORTH), coords)
            .expect("Place first piece");

        assert_eq!(board.can_place_at(coords), Err(PlaceError::Taken));
    }
}
