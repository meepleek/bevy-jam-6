use bevy::platform::collections::HashMap;
use bevy::prelude::*;

use super::Coords;
use super::piece::Piece;

const DEFAULT_SIZE: u16 = 6;

#[derive(Debug)]
pub struct Board {
    width: u16,
    heigth: u16,
    tiles: HashMap<Coords, Piece>,
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
        }
    }

    pub fn can_place_at(&self, coords: Coords) -> Result<(), PlaceError> {
        if coords.x >= self.width || coords.y >= self.heigth {
            return Err(PlaceError::OutOfBounds);
        } else if self.tiles.contains_key(&coords) {
            return Err(PlaceError::Taken);
        }

        Ok(())
    }

    pub fn place_piece(&mut self, coords: Coords, piece: Piece) -> Result<(), PlaceError> {
        self.can_place_at(coords)?;
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
    #[test_case(50, 0 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 6 => matches Err(PlaceError::OutOfBounds))]
    #[test_case(0, 50 => matches Err(PlaceError::OutOfBounds))]
    fn can_place_at_coords(x: u16, y: u16) -> Result<(), PlaceError> {
        let board = Board::new(6, 9);
        // L piece
        board.can_place_at((x, y).into())
    }

    #[test]
    fn cannot_place_at_coords_when_taken() {
        let coords: Coords = (3, 3).into();
        let mut board = Board::new(6, 6);
        board
            .place_piece(coords, Piece::Direction(Dir2::NORTH))
            .expect("Place first piece");

        assert_eq!(board.can_place_at(coords), Err(PlaceError::Taken));
    }
}
