use crate::prelude::*;

pub fn plugin(app: &mut App) {
    // app.add_observer(observer);
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
