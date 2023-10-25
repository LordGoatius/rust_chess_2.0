// Imports
use std::fmt::{Debug, Display};

use crate::piece::*;

// Custom Board Type Data
#[derive(Debug, Clone, Copy)]
pub struct Board {
    pub board: [[Option<Piece>; 8]; 8],
}

// Custom Board Type Traits

impl Default for Board {
    fn default() -> Self {
        Board { 
            board: build_starting_board(), 
        }
    }
}

impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut row_num: i8 = 8;
        let mut board_str: String = "   ┌───┬───┬───┬───┬───┬───┬───┬───┐\n".to_owned();
        for row in &self.board {
            let mut to_append: String = format!(" {row_num} │");
            row_num -= 1;
            for piece in row {
                match piece {
                    None => to_append.push_str("   │"),
                    Some(piece) => to_append.push_str(&format!(" {piece} │")[..]),
                }
            }
            board_str.push_str(&to_append[..]);
            board_str.push('\n');
            if row_num != 0 {
                board_str.push_str("   ├───┼───┼───┼───┼───┼───┼───┼───┤\n")
            }
        }
        board_str.push_str("   └───┴───┴───┴───┴───┴───┴───┴───┘\n");
        board_str.push_str("     A   B   C   D   E   F   G   H  \n");
        write!(f, "{board_str}")
    }
}

impl Board {
    pub fn move_piece(&mut self, start: Coordinates, end: Coordinates) {
        self.board[end.0 as usize][end.1 as usize] = self.board[start.0 as usize][start.1 as usize];
        self.board[start.0 as usize][start.1 as usize] = None;

        self.board[end.0 as usize][end.1 as usize].as_mut().unwrap().coordinates = end;
        match &mut self.at(end).as_mut().unwrap().piece_type {
            PieceType::King(val) => *val = false,
            PieceType::Rook(val) => *val = false,
            PieceType::Pawn(data) => *data = PawnData { can_en_pessant: false, has_moved: true },
            _ => (),
        }
    }

    pub fn swap_pieces(&mut self, start: Coordinates, end: Coordinates) {
        let temp = self.board[end.0 as usize][end.1 as usize];
        self.board[end.0 as usize][end.1 as usize] = self.board[start.0 as usize][start.1 as usize];
        self.board[start.0 as usize][start.1 as usize] = temp;

        self.board[end.0 as usize][end.1 as usize].as_mut().unwrap().coordinates = start;
        self.board[start.0 as usize][start.1 as usize].as_mut().unwrap().coordinates = end;

        if let PieceType::Rook(val) = &mut self.at(start).as_mut().unwrap().piece_type {
            *val = true;
        }

        if let PieceType::King(val) = &mut self.at(end).as_mut().unwrap().piece_type {
            *val = true;
        }
    }

    pub fn at(&mut self, coords: Coordinates) -> &mut Option<Piece> {
        &mut self.board[coords.0 as usize][coords.1 as usize]
    }
}

// Functions

fn build_starting_board() -> [[Option<Piece>; 8]; 8] {
    let mut board = [
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
        [None, None, None, None, None, None, None, None],
    ];
    board[0][0] = build_piece(PieceType::Rook(false), PieceColor::Black, Coordinates(0,0));
    board[0][1] = build_piece(PieceType::Knight,      PieceColor::Black, Coordinates(0,1));
    board[0][2] = build_piece(PieceType::Bishop,      PieceColor::Black, Coordinates(0,2));
    board[0][3] = build_piece(PieceType::Queen,       PieceColor::Black, Coordinates(0,3));
    board[0][4] = build_piece(PieceType::King(false), PieceColor::Black, Coordinates(0,4));
    board[0][5] = build_piece(PieceType::Bishop,      PieceColor::Black, Coordinates(0,5));
    board[0][6] = build_piece(PieceType::Knight,      PieceColor::Black, Coordinates(0,6));
    board[0][7] = build_piece(PieceType::Rook(false), PieceColor::Black, Coordinates(0,7));

    for i in 0..8 {
        board[1][i] = build_piece(PieceType::Pawn(PawnData::default()), PieceColor::Black, Coordinates(1,i as u8));
        board[6][i] = build_piece(PieceType::Pawn(PawnData::default()), PieceColor::White, Coordinates(6,i as u8))
    }

    for i in 2..6 {
        for j in 0..8 {
            board[i][j] = None;
        }
    }

    board[7][0] = build_piece(PieceType::Rook(false), PieceColor::White, Coordinates(7,0));
    board[7][1] = build_piece(PieceType::Knight,      PieceColor::White, Coordinates(7,1));
    board[7][2] = build_piece(PieceType::Bishop,      PieceColor::White, Coordinates(7,2));
    board[7][3] = build_piece(PieceType::Queen,       PieceColor::White, Coordinates(7,3));
    board[7][4] = build_piece(PieceType::King(false), PieceColor::White, Coordinates(7,4));
    board[7][5] = build_piece(PieceType::Bishop,      PieceColor::White, Coordinates(7,5));
    board[7][6] = build_piece(PieceType::Knight,      PieceColor::White, Coordinates(7,6));
    board[7][7] = build_piece(PieceType::Rook(false), PieceColor::White, Coordinates(7,7));
    board
}
