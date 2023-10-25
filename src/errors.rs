use std::{error::Error, fmt::Display, i8};
use crate::piece::Coordinates;

// GameError

#[derive(Debug)]
pub enum GameError {
    GenericError,
    NoPieceOnStartSquare,
    SameColorCapture,
    InvalidMove,
    InvalidMoveCheck,
    // TODO: Add MORE
}

impl Error for GameError {}

impl Display for GameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            GameError::GenericError => {
                write!(f, "Generic Placeholder Error")
            },
            GameError::NoPieceOnStartSquare => {
                write!(f, "Move cannot be made with no piece selected")
            },
            GameError::SameColorCapture => {
                write!(f, "Cannot capture same color")
            },
            GameError::InvalidMove => {
                write!(f, "Move is not valid/legal")
            },
            GameError::InvalidMoveCheck => {
                write!(f, "King is in check, cannot move")
            },
        }
    }
}

// GameResult 

#[derive(Debug)]
pub enum GameResult {
    Castle,
    EnPessant,
    DoublePawn,
    Promotion,
    Normal,
    Capture,
}

#[derive(Debug)]
pub struct ConversionError;

impl Display for ConversionError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Conversion Error")
    }
}

impl Error for ConversionError {}

pub trait Convertable {
    fn convert_to_u8(&self) -> Result<u8, Box<dyn Error>>;
}

impl Convertable for i8 {
    fn convert_to_u8(&self) -> Result<u8, Box<dyn Error>> {
        if *self < 0 {
            return Err(Box::new(ConversionError));
        } else {
            return Ok(*self as u8)
        }
    }
}
