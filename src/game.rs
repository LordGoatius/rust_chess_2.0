use core::fmt::Display;
use std::cmp;
use std::error::Error;
use std::io;

use crate::board::*;
use crate::piece::*;
use crate::errors::*;

// Game struct

#[derive(Debug, Copy, Clone)]
pub struct Game {
    pub board: Board,
    pub turn: PieceColor,
    pub move_num: u8,
    pub turn_num: u8,
}

// Impl Traits for Game

impl Default for Game {
    fn default() -> Self {
        Game {
            board: Board::default(),
            turn: PieceColor::White,
            move_num: 0,
            turn_num: 0,
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // write!(f, "{}       Turn: {}\nTurn Number: {}\n", self.board, self.turn, self.turn_num)
        write!(f, "{}", self.board)
    }
}

pub fn coordinate_from_input() -> Result<Option<Coordinates>, Box<dyn Error>> {
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)?;

    if input == "0".to_owned() {
        return Ok(None);
    }

    let coords = chess_notation_to_array_notation(&input[..])?;
    Ok(Some(coords))
}

// Impl the actual Game

impl Game {
    pub fn game_loop(&mut self) {
        'game_loop: loop {
            println!("{}", self.board);

            let mut input = String::new();

            println!("Print or Move");
            io::stdin()
                .read_line(&mut input)
                .expect("IO error");

            if input.to_ascii_lowercase().contains("print") {
                println!("Which square would you like to debug");
                let print = coordinate_from_input().unwrap().unwrap();
                println!("{:?}", self.at(print));
                continue;
            }

            let turn_color: PieceColor = if self.move_num % 2 == 0 {
                PieceColor::White
            } else {
                PieceColor::Black
            };

            // TODO check if move out of check is necessary
            
            let mut check = self.is_king_in_check(turn_color);
            
            println!("Input start square {turn_color}");
            let start_move = coordinate_from_input();
            let start_move_coords: Coordinates;
            match start_move {
                Ok(opt) => {
                    match opt {
                        None => break 'game_loop,
                        Some(val) => start_move_coords = val,
                    }
                },
                Err(_) => continue 'game_loop,
            }
            
            println!("Input end square {turn_color}");
            let end_move = coordinate_from_input();
            let end_move_coords: Coordinates;
            match end_move {
                Ok(opt) => {
                    match opt {
                        None => break 'game_loop,
                        Some(val) => end_move_coords = val,
                    }
                },
                Err(_) => continue 'game_loop,
            }

            if let Some(val) = self.at(start_move_coords) {
                if val.color != turn_color {
                    println!("Cannot move piece not owned by you");
                    continue 'game_loop;
                }
            } else {
                println!("Cannot move no piece");
                continue 'game_loop;
            }

            // TODO if move out of check is necessary, make sure move moves out of check
            // if it doesn't continue the loop

            if check {
                let mut test = self.clone();
                let _ = test.make_move(start_move_coords, end_move_coords);
                if test.is_king_in_check(turn_color) {
                    println!("Must move out of check");
                    continue;
                }
            }

            // We KNOW:
            //     Correct piece color
            //     Not an empty spot
            //     Not check
            // We Must:
            //     Validate the move
            //     Check for checkmate
            
            if let Some(piece) = self.at(start_move_coords) {
                if let PieceType::Pawn(_) = piece.piece_type {
                    match turn_color {
                        PieceColor::Black => {
                            if start_move_coords.0 > end_move_coords.0 {
                                println!("Illegal Move");
                                continue;
                            }
                        },
                        PieceColor::White => {
                            if start_move_coords.0 < end_move_coords.0 {
                                println!("Illegal Move");
                                continue;
                            }
                        },
                    }
                }
            }

            match self.make_move(start_move_coords, end_move_coords) {
                Err(error) => {
                    println!("{error}");
                    continue;
                },
                Ok(game_result) => {
                    match game_result {
                        GameResult::Promotion => {
                            self.handle_promotion(end_move_coords);
                        }
                        _ => {},
                    }
                }
            }

            // checks if move checkmates the enemy king
            if self.is_king_in_check(turn_color.swap()) && (self.check_checkmate(turn_color) == true) {
                println!("{turn_color} Wins!");
                break;
            }

            self.move_num += 1;
            self.turn_num = self.move_num/2;
        }
    }

    pub fn is_king_in_check(&mut self, color: PieceColor) -> bool {
        for row in self.board.board {
            for piece_option in row {
                if let Some(piece) = piece_option {
                    if let PieceType::King(_) = piece.piece_type {
                        if color == piece.color {
                            if self.is_coord_attacked_by_team(color.swap(), piece.coordinates) {
                                return true;
                            }
                        }
                    }
                }
            }
        }
        false
    }

    /// Checks if the king of the opposite color is in checkmate
    pub fn check_checkmate(&self, turn_color: PieceColor) -> bool {
        let mut enemy_king_coords = Coordinates(0, 0);

        for row in self.board.board {
            for piece_option in row {
                if let Some(piece) = piece_option {
                    if let PieceType::King(_) = piece.piece_type {
                        if turn_color.swap() == piece.color {
                            enemy_king_coords = piece.coordinates;
                        }
                    }
                }
            }
        }
        for i in -1i8..=1 {
            for j in -1i8..=1 {
                let mut cloned_board = self.clone();
                let status = cloned_board.make_move(enemy_king_coords, Coordinates(cmp::max(cmp::min(0, enemy_king_coords.0 as i8 + i), 7) as u8, cmp::max(cmp::min(0, enemy_king_coords.0 as i8 + j), 7) as u8));
                if let Ok(_) = status {
                    if !cloned_board.is_king_in_check(turn_color.swap()) {
                        return false;
                    }
                }

            }
        }
        true
    }

    pub fn handle_promotion(&mut self, coords: Coordinates) {
        todo!()
    }

    pub fn make_move(&mut self, start: Coordinates, end: Coordinates) -> Result<GameResult, GameError> {
        // Function Guards
        if &None == self.at(start) {
            return Err(GameError::NoPieceOnStartSquare);
        }

        // Handled castling here
        if let PieceType::King(false) = self.at(start).unwrap().piece_type {
            if let Some(_) = self.at(end) {
                if let PieceType::Rook(false) = self.at(end).unwrap().piece_type {
                    if !self.empty_between(start, end) && !self.is_coord_range_attacked_by_team(self.clone().at(start).unwrap().color, start, end){
                        return Err(GameError::InvalidMove);
                    }
                    self.board.swap_pieces(start, end);
                    return Ok(GameResult::Castle);
                }
            }
        }

        if let &mut Some(piece) = self.at(end) {
            if piece.color == self.at(start).unwrap().color {
                return Err(GameError::SameColorCapture);
            }
        }

        // All pawn movement (en pessant, double first) handled here
        // TODO FORWARD MOVEMENT MUST BE HANDLED BY CALLEE FUNCTION
        if let PieceType::Pawn(data) = self.at(start).unwrap().piece_type {
            // Double First Move
            if (end.0 as i8 - start.0 as i8).abs() == 2 && 
               (end.1 as i8 - start.1 as i8).abs() == 0 &&
               data.has_moved == false &&
               self.empty_between(start, end) {
                   if let Some(piece_1) = &mut self.at(Coordinates(end.0, (end.1 as i8 - 1).abs() as u8)).as_mut() {
                        match piece_1.piece_type {
                           PieceType::Pawn(_) => {
                               self.at(Coordinates(end.0, (end.1 as i8 - 1).abs() as u8)).as_mut().unwrap().piece_type =
                                   PieceType::Pawn(PawnData { has_moved: true, can_en_pessant: true });
                           },
                           _ => (),
                       }
                   }
                   if let Some(piece_2) = &mut self.at(Coordinates(end.0, cmp::min((end.1 as i8 + 1).abs() as u8, 7))).as_mut() {
                       match piece_2.piece_type {
                           PieceType::Pawn(_) => {
                               self.at(Coordinates(end.0, (end.1 as i8 + 1).abs() as u8)).as_mut().unwrap().piece_type =
                                   PieceType::Pawn(PawnData { has_moved: true, can_en_pessant: true });
                           },
                           _ => (),
                       }
                   }
                   self.board.move_piece(start, end);
                   return Ok(GameResult::DoublePawn);
            }
            // En Pessant
            else if (end.0 as i8 - start.0 as i8).abs() == 1 && 
                    (end.1 as i8 - start.1 as i8).abs() == 1 && 
                    data.can_en_pessant == true && 
                    self.at(end) == &None {
                if let &mut Some(piece) = self.at(Coordinates(start.0, end.1)) {
                    if let PieceType::Pawn(_) = piece.piece_type {
                        if piece.color != self.at(start).unwrap().color {
                            *self.board.at(Coordinates(start.0, end.1)) = None;
                            self.board.move_piece(start, end);
                            return Ok(GameResult::EnPessant);
                        }
                    }
                }
            }
            // Normal Capture
            else if (end.0 as i8 - start.0 as i8).abs() == 1 && 
                    (end.1 as i8 - start.1 as i8).abs() == 1 {
                if let &mut Some(_) = self.at(end) {
                    self.board.move_piece(start, end);
                    self.at(end).as_mut().unwrap().piece_type =
                        PieceType::Pawn(PawnData { has_moved: true, can_en_pessant: false });
                    if end.0 == 7 || end.0 == 0 {
                        return Ok(GameResult::Promotion)
                    }
                    return Ok(GameResult::Capture);
                }
            }
            // Normal Move 
            else if (end.0 as i8 - start.0 as i8).abs() == 1 && 
                    (end.1 as i8 - start.1 as i8).abs() == 0 && 
                    self.at(end) == &None {
                self.board.move_piece(start, end);
                self.at(end).as_mut().unwrap().piece_type =
                    PieceType::Pawn(PawnData { has_moved: true, can_en_pessant: false });
                if end.0 == 7 || end.0 == 0 {
                    return Ok(GameResult::Promotion)
                }
                return Ok(GameResult::Normal);
            }
            return Err(GameError::GenericError);
        }

        // TODO valid_move should check checkmate, and return check if needed, not just InvalidMove
        if let Err(error) = self.valid_move(start, end) {
            return Err(error);
        }

        match &mut self.at(start).as_mut().unwrap().piece_type {
            PieceType::Pawn(data) => {
                data.has_moved = true;
            },
            PieceType::Rook(data) => {
                *data = true;
            },
            PieceType::King(data) => {
                *data = true;
            },
            _ => (),
        }

        // We know the contained piece is not the same color because it would have returned error
        // otherwise
        if let Some(_) = self.at(end) {
            self.board.move_piece(start, end);
            Ok(GameResult::Capture)
        } else {
            self.board.move_piece(start, end);
            Ok(GameResult::Normal)
        }
    }

    pub fn at(&mut self, coords: Coordinates) -> &mut Option<Piece> {
        self.board.at(coords)
    }

    // Piece exists, is attacking different color (or) None
    fn valid_move(&mut self, start: Coordinates, end: Coordinates) -> Result<GameResult, GameError> {
        if let None = self.at(start) {
            return Err(GameError::NoPieceOnStartSquare);
        }

        match self.at(start).unwrap().piece_type {
            PieceType::Queen => {
                if self.is_piece_attacking_coordinates(PieceType::Queen, start, end) {
                    return Ok(GameResult::Normal);
                } else {
                    return Err(GameError::InvalidMove);
                }
            },
            PieceType::Bishop => {
                if self.is_piece_attacking_coordinates(PieceType::Bishop, start, end) {
                    return Ok(GameResult::Normal);
                } else {
                    return Err(GameError::InvalidMove);
                }
            },
            PieceType::Knight => {
                if self.is_piece_attacking_coordinates(PieceType::Knight, start, end) {
                    return Ok(GameResult::Normal);
                } else {
                    return Err(GameError::InvalidMove);
                }
            },
            PieceType::Rook(val) => {
                if self.is_piece_attacking_coordinates(PieceType::Rook(val), start, end) {
                    return Ok(GameResult::Normal);
                } else {
                    return Err(GameError::InvalidMove);
                }
            },
            PieceType::King(val) => {
                let color = self.at(start).unwrap().color.swap();
                if self.is_piece_attacking_coordinates(PieceType::King(val), start, end) &&
                   !self.is_coord_attacked_by_team(color, end) {
                       return Ok(GameResult::Normal);
                   } else {
                       return Err(GameError::InvalidMoveCheck)
                   }
            },
            PieceType::Pawn(_) => {
                // TODO should never be called probably
            },
        }
        Ok(GameResult::Normal)
    }

    pub fn is_coord_range_attacked_by_team(&mut self, color: PieceColor, start: Coordinates, end: Coordinates) -> bool {
        if !(start.0 == end.0 || start.1 == end.1) {
            return false;
        }

        if start.0 == end.0 {
            for square in coord_range(start.1.into(), end.1.into()) {
                if self.is_coord_attacked_by_team(color, Coordinates(start.0, square as u8)) {
                    return true;
                }
            }
        }

        if start.1 == end.1 {
            for square in coord_range(start.0.into(), end.0.into()) {
                if self.is_coord_attacked_by_team(color, Coordinates(square as u8, start.1)) {
                    return true;
                }
            }
        }
        false
    }

    pub fn is_coord_attacked_by_team(&mut self, color: PieceColor, coords: Coordinates) -> bool {
        for row in self.board.board {
            for value in row.iter().filter(|row_val| {
                if let None = row_val {
                    return false;
                } else {
                    if row_val.unwrap().color == color {
                        return true;
                    }
                    false
                }
            }) {
                if self.is_piece_attacking_coordinates(value.unwrap().piece_type, value.unwrap().coordinates, coords) {
                    return true;
                }
            }
        }
        false
    }

    pub fn empty_between(&mut self, start: Coordinates, end: Coordinates) -> bool {
        // return false if not straight line
        if !(start.0 == end.0 || start.1 == end.1) {
            return false;
        }

        if start.0 == end.0 {
            for square in coord_range(start.1.into(), end.1.into()) {
                if let Some(_) = self.at(Coordinates(start.0, square as u8)) {
                    return false;
                }
            }
        }

        if start.1 == end.1 {
            for square in coord_range(start.0.into(), end.0.into()) {
                if let Some(_) = self.at(Coordinates(square as u8, start.1)) {
                    return false;
                }
            }
        }
        true
    }

    pub fn empty_between_diag(&mut self, start: Coordinates, end: Coordinates) -> bool {
        // return false if not diagonal
        if (start.0 as i8 - end.0 as i8).abs() != (start.1 as i8 - end.1 as i8).abs() {
            return false;
        }
        
        for (s, e) in coord_range(start.0.into(), end.0.into()).zip(coord_range(start.1.into(), end.1.into())) {
            if let Some(_) = self.at(Coordinates(s as u8, e as u8)) {
                return false;
            }
        }
        true
    }

    pub fn is_piece_attacking_coordinates(&mut self, piece: PieceType, piece_coords: Coordinates, attacking_coords: Coordinates) -> bool {
        match piece{            
            PieceType::King(_) => {
                (attacking_coords.0 as i8 - piece_coords.0 as i8).abs() <= 1 &&
                (attacking_coords.1 as i8 - piece_coords.1 as i8).abs() <= 1
            },
            PieceType::Rook(_) => {
                self.empty_between(piece_coords, attacking_coords)
            },
            PieceType::Queen => {
                self.empty_between(piece_coords, attacking_coords) ||
                self.empty_between_diag(piece_coords, attacking_coords)
            },
            PieceType::Knight=> {
                ((piece_coords.0 as i8 - attacking_coords.0 as i8).abs() == 2 && (piece_coords.1 as i8 - attacking_coords.1 as i8).abs() == 1) ||
                ((piece_coords.0 as i8 - attacking_coords.0 as i8).abs() == 1 && (piece_coords.1 as i8 - attacking_coords.1 as i8).abs() == 2)
            },
            PieceType::Bishop=> {
                self.empty_between_diag(piece_coords, attacking_coords)
            },
            // Pawn will probably never be called, but in case it will be-
            // It should cover en pessant, so if the function is used
            // to determind where any piece can go, it should be highlighted
            PieceType::Pawn(_) => { false },
        }
    }
}

// Util Functions

pub fn chess_notation_to_array_notation(chess_not: &str) -> Result<Coordinates, Box<dyn Error>> /* file is columns*/ {
    let file = chess_not.chars().nth(0).unwrap().to_ascii_uppercase();
    let file_u8: u8 = u8::try_from(file)?.wrapping_add_signed(-65);
    let rank = chess_not.chars().nth(1).unwrap().to_digit(10).ok_or(ConversionError)?;

    if file_u8 > 7 || rank > 8 {
        return Err(Box::new(ConversionError));
    }

    let rank_u8: u8 = 8 - u8::try_from(rank)?;

    Ok(Coordinates(rank_u8, file_u8))
}

pub fn coord_range(first: usize, second: usize) -> Box<dyn Iterator<Item = i32>> {
    if first > second {
        Box::new(((second as i32 + 1)..(first as i32)).rev())
    } else {
        Box::new((first as i32 + 1)..(second as i32)) as Box<dyn Iterator<Item = i32>>
    }
}
