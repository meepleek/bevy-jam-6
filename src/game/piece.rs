use bevy::prelude::*;

const PIECE_SIZE: u16 = 5;

#[derive(Debug, Clone)]
pub enum Piece {
    Pattern([u16; (PIECE_SIZE * PIECE_SIZE) as usize]),
    Direction(Dir2),
}

// impl std::fmt::Display for Piece {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str(data)
//     }
// }
