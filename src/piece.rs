// Imports
use std::fmt::{Debug, Display};

// Custom Piece Type Data

#[derive(Debug, PartialEq, Copy, Clone)]
pub enum PieceColor {
    Black,
    White,
}

impl PieceColor {
    pub fn swap(&self) -> PieceColor {
        match self {
            PieceColor::White => PieceColor::Black,
            PieceColor::Black => PieceColor::White,
        }
    }
}

#[derive(Debug, Default, Copy, Clone, PartialEq)]
pub struct PawnData {
    pub has_moved: bool,
    pub can_en_pessant: bool,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub enum PieceType {
    Pawn(PawnData),
    Rook(bool),
    King(bool),
    Bishop,
    Knight,
    Queen,
}

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Coordinates(pub u8, pub u8);

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Piece {
    pub piece_type: PieceType,
    pub color: PieceColor,
    pub coordinates: Coordinates, 
}

// Custom Piece Type Traits

impl Display for Piece {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let caps: bool = self.color == PieceColor::White;
        let mut letter: char = match &self.piece_type {
            PieceType::Pawn(_) => {
                'p'
            },
            PieceType::Rook(_) => {
                'r'
            },
            PieceType::King(_) => {
                'k'
            },
            PieceType::Bishop => {
                'b'
            },
            PieceType::Knight => {
                'n'
            },
            PieceType::Queen =>
                'q'
        };
        if caps {
            letter = letter.to_ascii_uppercase();
        }
        write!(f, "{letter}")
    }
}

impl Display for Coordinates {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.0, self.1)
    }
}

impl Display for PieceColor {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            PieceColor::Black => {
                write!(f, "Black")
            },
            PieceColor::White => {
                write!(f, "White")
            },
        }
    }
}

// Custom Functions

pub fn build_piece(piece: PieceType, color: PieceColor, coordinates: Coordinates) -> Option<Piece> {
    let to_build = Piece {
        piece_type: piece,
        color,
        coordinates,
    };
    let ret: Option<Piece> = Some(to_build);
    return ret;
}
