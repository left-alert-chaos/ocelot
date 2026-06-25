//!# types
//!This module holds structs and enums used for movement.

use std::fmt;
use crate::physical::board::{self, Board, Coordinate, Piece};

///# Action
///Implemented by Move and Castle.
///Display should display the classical coordinate instead of 0-indexed coordinate.
pub trait Action: fmt::Debug + fmt::Display {
    fn perform_on(&mut self, game: &mut Board); //requires mutablility because it records capture
    //information to restore in undo()
    fn undo_on(&self, game: &mut Board);
    fn to_coordinate(&self) -> Option<Coordinate>;
    fn is_illegal(&mut self, game: &mut Board) -> bool;
    fn duplicate(&self) -> Box<dyn Action>;
    fn evaluate(&self) -> f64;
}

///# Move
///Holds information about a move and implements Action, so it can be performed.
///
///# Public Methods
///
///## new(from: Coordinate, to: board::Coordinate, en_passant: Option<board::Coordinate>,
///value: u32, promotion: Option<board::PieceType>)
#[derive(Copy, Clone)]
pub struct Move {
    from: Coordinate,
    to: Coordinate,
    en_passant: Option<Coordinate>,
    value: u32,
    promotion: Option<board::PieceType>,
    captured_type: Option<board::PieceType>,
    piece_first_move: bool,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Move from {} to {}",
            self.from, self.to
        )
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Move from {:?} to {:?} with value {} [DEBUG]", self.from, self.to, self.value)
    }
}

impl Action for Move {
    fn to_coordinate(&self) -> Option<Coordinate> {
        Some(self.to)
    }

    fn is_illegal(&mut self, game: &mut Board) -> bool {
        //unwrap moving piece
        let moving_piece = match game.square(&self.from).piece {
            Some(piece) => piece,
            None => {
                eprintln!(
                    "Move::is_illegal(): Returning false because there is no piece where this move is from ({:?})",
                    self.from
                );
                return false;
            }
        };

        //check for check
        self.perform_on(game);
        let result = game.is_check(moving_piece.color);
        self.undo_on(game);

        result
    }

    //Doesn't use game.move_from() because it might need to modify the piece before it moves.
    fn perform_on(&mut self, game: &mut Board) {
        let from_square = game.mut_square(&self.from);
        let mut moving_piece = from_square.piece.unwrap_or_else(|| {
            panic!("{self} isn't legal because it is from a square without a piece.")
        });
        moving_piece.has_moved = true;

        //handle promotion
        if let Some(promo_type) = self.promotion {
            moving_piece.ptype = promo_type;
        }

        //record capture
        let to_square = game.square(&self.to);
        if let Some(captured_piece) = to_square.piece {
            self.captured_type = Some(captured_piece.ptype);
        }

        //increment round
        game.turn = moving_piece.color.opposite();
        if game.turn == board::Color::White {
            game.round += 1;
        }

        //if is 2 square pawn move, set en passant-ability
        if moving_piece.ptype.value() == 1 && self.is_two_square_pawn_move() {
            //if white, black captures on same turn. If black, white captures next round
            let turn = if moving_piece.color == board::Color::White {
                game.round
            } else {
                game.round + 1
            };
            moving_piece.ptype = board::PieceType::Pawn(turn);
        }

        //perform move
        game.remove_on(&self.from);
        game.put_piece_on(&self.to, moving_piece);

        //lastly, delete the piece on the en_passant square
        if let Some(ep_loc) = self.en_passant {
            game.remove_on(&ep_loc);
        }

        //update board
        game.update();
    }

    //basically the same logic as perform_on(), but in reverse
    fn undo_on(&self, game: &mut Board) {
        game.round -= 1;
        let from_square = game.mut_square(&self.to);
        let mut moving_piece = from_square.piece.unwrap_or_else(|| {
            panic!("Undoing {self} isn't possible because there is no piece on the to square.")
        });

        //handle un-promoting
        if self.promotion.is_some() {
            moving_piece.ptype = board::PieceType::Pawn(0); //promotion doesn't matter; just use garbage.
        }

        //set has_moved
        moving_piece.has_moved = !self.piece_first_move;

        //if is 2 square pawn move, set en passant-ability
        if moving_piece.ptype.value() == 1 && self.is_two_square_pawn_move() {
            moving_piece.ptype = board::PieceType::Pawn(0);
        }

        //de-increment round
        game.turn = game.turn.opposite();
        if game.turn == board::Color::Black {
            game.round -= 1;
        }

        //perform un-move
        game.remove_on(&self.to);
        game.put_piece_on(&self.from, moving_piece);

        //restore en passant
        if let Some(ep_loc) = self.en_passant {
            game.put_piece_on(
                &ep_loc,
                Piece {
                    color: if moving_piece.color == board::Color::White {
                        board::Color::Black
                    } else {
                        board::Color::White
                    },
                    ptype: board::PieceType::Pawn(game.round),
                    location: ep_loc,
                    has_moved: false,
                },
            );
        }

        //restore capture
        if let Some(captured_type) = self.captured_type {
            let restored_piece = Piece {
                color: if moving_piece.color == board::Color::White {
                    board::Color::Black
                } else {
                    board::Color::White
                },
                ptype: captured_type,
                location: self.to,
                has_moved: false,
            };
            game.put_piece_on(&self.to, restored_piece);
        }


        game.update();
    }

    fn duplicate(&self) -> Box<dyn Action> {
        Box::new(Self {
            ..*self
        })
    }

    fn evaluate(&self) -> f64 {
        self.value as f64
    }
}

impl PartialEq for Move {
    fn eq(&self, other: &Self) -> bool {
        self.from == other.from && self.to == other.to
    }
}

impl Move {
    pub fn new(
        from: Coordinate,
        to: board::Coordinate,
        game: &Board,
        promotion: Option<board::PieceType>,
        en_passant: bool,
    ) -> Self {
        //get info about move
        let mut piece_first_move = false;
        let mut moving_piece_color = None;
        let moving_piece = game.square(&from).piece;
        if let Some(piece) = moving_piece {
            piece_first_move = !piece.has_moved;
            moving_piece_color = Some(piece.color);
        } else {
            eprintln!("WARNING: Creating move from {from}, which doesn't have a piece.");
        }

        //check if en passant is possible
        let en_passant_location = if en_passant {
            //direction of en passant, NOT pawn movement direction.
            let direction: i32 = if from.row < to.row { -1 } else { 1 };
            let loc = to.with_offset(direction, 0).unwrap();
            
            //check if can be en passant'ed
            match game.square(&loc).piece {
                Some(piece) => {
                    //can it be en passant'ed this turn?
                    if piece.ptype == board::PieceType::Pawn(game.round) && Some(piece.color) != moving_piece_color {
                        Some(loc)
                    } else {
                        None
                    }
                }
                None => {
                    None
                }
            }
        } else {
            None
        };

        //do a quick-and-dirty evaluation. This is overwritten by search, but useful in some places.
        let value = if let Some(ptype) = promotion {
            ptype.value() - 1
        } else {
            //if there is a piece, get the value of that piece. otherwise, no value.
            let to_square = game.square(&to);
            if let Some(piece) = to_square.piece {
                piece.ptype.value()
            } else {
                0
            }
        };

        Move {
            from,
            to,
            en_passant: en_passant_location,
            value: value as u32,
            promotion,
            captured_type: None,
            piece_first_move,
        }
    }

    fn is_two_square_pawn_move(&self) -> bool {
        let mut rise = self.to.row as i32 - self.from.row as i32;
        rise = rise.abs();

        rise == 2
    }
}

///# CastleSide
///Represents the side a player can castle on.ype.
#[derive(Copy, Clone)]
pub enum CastleSide {
    KingSide,
    QueenSide,
}

impl fmt::Display for CastleSide {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let phrase = match self {
            Self::KingSide => "King Side",
            Self::QueenSide => "Queen Side",
        };
        write!(f, "{phrase}")
    }
}

impl CastleSide {
    fn rook_start_col(&self) -> usize {
        match self {
            Self::KingSide => 7,
            Self::QueenSide => 0,
        }
    }

    fn rook_end_col(&self) -> usize {
        match self {
            Self::KingSide => 5,
            Self::QueenSide => 3,
        }
    }

    fn king_end_col(&self) -> usize {
        match self {
            Self::KingSide => 6,
            Self::QueenSide => 2,
        }
    }

    //I love this function name
    fn squares_that_must_be_empty(&self) -> Vec<usize> {
        match self {
            Self::KingSide => vec![6, 5],
            Self::QueenSide => vec![1, 2, 3],
        }
    }
}

///# Castle
///Represents a castling move. Implements Action.
///
///# Public Methods
///
///## new(side: CastleSide, player: board::Color) -> Self
///Simple abstraction to functionally instantiate the struct.
#[derive(Clone, Copy)]
pub struct Castle {
    side: CastleSide,
    player: board::Color,
}

impl fmt::Display for Castle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{} {} castles", self.player, self.side)
    }
}

impl fmt::Debug for Castle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self} [DEBUG]")
    }
}

impl Action for Castle {
    fn to_coordinate(&self) -> Option<Coordinate> {
        None
    }

    fn perform_on(&mut self, game: &mut Board) {
        let row = self.row();

        let king_pos = Coordinate::new(4, row);
        let king_target = Coordinate::new(self.side.king_end_col(), row);
        let rook_pos = Coordinate::new(self.side.rook_start_col(), row);
        let rook_target = Coordinate::new(self.side.rook_end_col(), row);

        game.move_from(&king_pos, &king_target);
        game.move_from(&rook_pos, &rook_target);
        game.update();
    }

    fn undo_on(&self, game: &mut Board) {
        let row = self.row();

        //do the same thing as above, but reversed.
        let king_target = Coordinate::new(4, row);
        let king_pos = Coordinate::new(self.side.king_end_col(), row);
        let rook_pos = Coordinate::new(self.side.rook_end_col(), row);
        let rook_target = Coordinate::new(self.side.rook_start_col(), row);

        game.move_from(&king_pos, &king_target);
        game.move_from(&rook_pos, &rook_target);

        //reset has_moved
        game.mut_square(&king_target).piece.unwrap().has_moved = false;
        game.mut_square(&rook_target).piece.unwrap().has_moved = false;
        game.update();
    }

    fn is_illegal(&mut self, game: &mut Board) -> bool {
        let row = self.row();

        //check whether the king is there and legal
        let king_loc = Coordinate::new(4, row);
        let expected_king = Piece {
            color: self.player,
            ptype: board::PieceType::King,
            location: king_loc,
            has_moved: false,
        };
        let king_square = game.square(&king_loc);
        let king = match king_square.piece {
            Some(king) => king,
            None => return true,
        };
        if king != expected_king {
            return true;
        }

        if game.is_check(king.color) {
            return true;
        }

        //check for rook
        let rook_loc = Coordinate::new(self.side.rook_start_col(), row);
        let expected_rook = Piece {
            color: self.player,
            ptype: board::PieceType::Rook,
            location: rook_loc,
            has_moved: false,
        };
        let rook_square = game.square(&rook_loc);
        let rook = match rook_square.piece {
            Some(rook) => rook,
            None => return true,
        };
        if rook != expected_rook {
            return true;
        }

        //check for empty and unattacked squares
        for col in self.side.squares_that_must_be_empty() {
            let coord = Coordinate::new(col, row);

            let opponent_threats = match self.player {
                board::Color::White => &game.move_info.black_threatened_squares,
                board::Color::Black => &game.move_info.white_threatened_squares,
            };

            if opponent_threats.contains(&coord) {
                return true;
            }

            let square = game.square(&coord);
            if square.piece.is_some() {
                return true;
            }
        }

        false
    }

    fn duplicate(&self) -> Box<dyn Action> {
        Box::new(Self {
            ..*self
        })
    }

    fn evaluate(&self) -> f64 {
        2.0
    }
}

impl Castle {
    pub fn new(side: CastleSide, player: board::Color) -> Self {
        Castle { side, player }
    }

    fn row(&self) -> usize {
        match self.player {
            board::Color::White => 0,
            board::Color::Black => 7,
        }
    }
}

///# MoveInfo
///Holds information about threatened squares, legal moves, etc. It exists to remove the need for
///generating moves more times than necessary.
///
///# Public Methods
///
///## new() -> Self
///Creates a new instance with empty Vecs in its fields.
///
///## from(game: &board::Board) -> Self
///Creates a MoveInfo instance with information generated from the board.
///
///## update(&mut self, game: &board::Board)
///Generates moves for game and updates self's fields with info.
#[derive(Debug)]
pub struct MoveInfo {
    pub(crate) white_threatened_squares: Vec<Coordinate>,
    pub(crate) black_threatened_squares: Vec<Coordinate>,
    pub(crate) white_potential_moves: Vec<Box<dyn Action>>,
    pub(crate) black_potential_moves: Vec<Box<dyn Action>>,
}

impl MoveInfo {
    pub fn new() -> Self {
        Self {
            white_threatened_squares: Vec::new(),
            black_threatened_squares: Vec::new(),
            white_potential_moves: Vec::new(),
            black_potential_moves: Vec::new(),
        }
    }

    pub fn from(game: &Board) -> Self {
        let mut info = Self::new();
        info.update(game);
        info
    }

    pub fn update(&mut self, game: &Board) {
        self.white_potential_moves = game.white_potential_moves();
        self.black_potential_moves = game.black_potential_moves();

        self.white_threatened_squares = Vec::new();
        for m in self.white_potential_moves.iter() {
            if let Some(coord) = m.to_coordinate() {
                self.white_threatened_squares.push(coord);
            }
        }

        self.black_threatened_squares = Vec::new();
        for m in self.black_potential_moves.iter() {
            if let Some(coord) = m.to_coordinate() {
                self.black_threatened_squares.push(coord);
            }
        }
    }
}

