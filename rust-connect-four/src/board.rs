#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceColor {
    Red,
    Yellow,
}

pub struct GamePiece {
    pub color: PieceColor,
}

impl GamePiece {
    pub fn new(color: PieceColor) -> GamePiece {
        GamePiece {
            color,
        }
    }
}

