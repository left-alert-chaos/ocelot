//!# movement
//!Handles impls for piece movement, move generation, legality checking, etc.

use crate::physical::board;
use board::{Board, Color, Coordinate, Piece, PieceType};
use std::fmt;

pub trait Action: fmt::Debug {
    fn perform_on(&mut self, game: &mut Board); //requires mutablility because it records capture
    //information to restore in undo()
    fn undo_on(&self, game: &mut Board);
    fn _is_illegal(&self, _game: &Board) -> bool {
        false
    }
}

///# Move
///Holds information about a move and implements Action, so it can be performed.
///
///# Public Methods
///
///## new(from: Coordinate, to: board::Coordinate, en_passant: Option<board::Coordinate>,
///value: u32, promotion: Option<PieceType>)
#[derive(Copy, Clone)]
pub struct Move {
    from: Coordinate,
    to: Coordinate,
    en_passant: Option<Coordinate>,
    value: u32,
    promotion: Option<PieceType>,
    captured_type: Option<PieceType>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "Move from {} to {} with value {}",
            self.from, self.to, self.value
        )
    }
}

impl fmt::Debug for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self} [DEBUG]")
    }
}

impl Action for Move {
    //Doesn't use game.move_from() because it might need to modify the piece before it moves.
    fn perform_on(&mut self, game: &mut Board) {
        let from_square = game.mut_square(&self.from);
        let mut moving_piece = from_square.piece.expect(
            format!("{self} isn't legal because it is from a square without a piece.").as_str(),
        );

        //handle promotion
        if let Some(promo_type) = self.promotion {
            moving_piece.ptype = promo_type;
        }

        //record capture
        let to_square = game.square(&self.to);
        if let Some(captured_piece) = to_square.piece {
            self.captured_type = Some(captured_piece.ptype);
        }

        //perform move
        game.remove_piece_on(&self.from);
        game.put_piece_on(&self.to, moving_piece);

        //lastly, delete the piece on the en_passant square
        if let Some(ep_loc) = self.en_passant {
            game.remove_piece_on(&ep_loc);
        }
    }

    //basically the same logic as perform_on(), but in reverse
    fn undo_on(&self, game: &mut Board) {
        let from_square = game.mut_square(&self.to);
        let mut moving_piece = from_square.piece.expect(
            format!("Undoing {self} isn't possible because there is no piece on the to square.")
                .as_str(),
        );

        //handle un-promoting
        if let Some(_) = self.promotion {
            moving_piece.ptype = PieceType::Pawn;
        }

        //perform un-move
        game.remove_piece_on(&self.to);
        game.put_piece_on(&self.from, moving_piece);

        //restore en passant
        if let Some(ep_loc) = self.en_passant {
            game.put_piece_on(
                &ep_loc,
                Piece {
                    color: if moving_piece.color == Color::White {
                        Color::Black
                    } else {
                        Color::White
                    },
                    ptype: PieceType::Pawn,
                    location: ep_loc,
                },
            );
        }

        //restore capture
        if let Some(captured_type) = self.captured_type {
            let restored_piece = Piece {
                color: if moving_piece.color == Color::White {
                    Color::Black
                } else {
                    Color::White
                },
                ptype: captured_type,
                location: self.to,
            };
            game.put_piece_on(&self.to, restored_piece);
        }
    }
}

impl Move {
    pub fn new(
        from: Coordinate,
        to: board::Coordinate,
        game: &Board,
        promotion: Option<PieceType>,
        en_passant: bool,
    ) -> Self {
        let en_passant_location = if en_passant {
            //direction of en passant, NOT pawn movement direction.
            let direction: i32 = if from.row < to.row { -1 } else { 1 };
            Some(Coordinate::new(to.col, to.row + direction as usize))
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
        }
    }
}

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

///# Castle
///Represents a castling move. Implements Action.
///
///# Public Methods
///
///## new(side: CastleSide, player: Color) -> Self
///Simple abstraction to functionally instantiate the struct.
pub struct Castle {
    side: CastleSide,
    player: Color,
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
    fn perform_on(&mut self, game: &mut Board) {
        let row = self.row();

        let king_pos = Coordinate::new(4, row);
        let king_target: Coordinate;
        let rook_pos: Coordinate;
        let rook_target: Coordinate;

        //Set current locations and target locations
        match self.side {
            CastleSide::KingSide => {
                king_target = Coordinate::new(6, row);
                rook_pos = Coordinate::new(7, row);
                rook_target = Coordinate::new(5, row);
            }
            CastleSide::QueenSide => {
                king_target = Coordinate::new(2, row);
                rook_pos = Coordinate::new(0, row);
                rook_target = Coordinate::new(3, row);
            }
        }

        game.move_from(&king_pos, &king_target);
        game.move_from(&rook_pos, &rook_target);
    }

    fn undo_on(&self, game: &mut Board) {
        let row = self.row();

        //do the same thing as above, but reversed.
        let king_target = Coordinate::new(4, row);
        let king_pos: Coordinate;
        let rook_pos: Coordinate;
        let rook_target: Coordinate;

        //set current locations and target locations
        match self.side {
            CastleSide::KingSide => {
                king_pos = Coordinate::new(6, row);
                rook_target = Coordinate::new(7, row);
                rook_pos = Coordinate::new(5, row);
            }
            CastleSide::QueenSide => {
                king_pos = Coordinate::new(2, row);
                rook_target = Coordinate::new(0, row);
                rook_pos = Coordinate::new(3, row);
            }
        }

        game.move_from(&king_pos, &king_target);
        game.move_from(&rook_pos, &rook_target);
    }
}

impl Castle {
    pub fn new(side: CastleSide, player: Color) -> Self {
        Castle { side, player }
    }

    fn row(&self) -> usize {
        match self.player {
            Color::White => 0,
            Color::Black => 7,
        }
    }
}

//seperate impl for move generation
impl Piece {
    pub fn potential_moves(&self, game: &Board) -> Vec<impl Action> {
        match self.ptype {
            PieceType::Rook => self.rook_moves(game),
            PieceType::Bishop => self.bishop_moves(game),
            PieceType::Queen => { //queen movement is just rook and bishop combined
                let mut moves = self.rook_moves(game);
                moves.append(&mut self.bishop_moves(game));
                moves
            }
            _ => Vec::new(),
        }
    }

    fn rook_moves(&self, game: &Board) -> Vec<Move> {
        let mut moves = self.project(game, 1, 0);
        moves.append(&mut self.project(game, -1, 0));
        moves.append(&mut self.project(game, 0, 1));
        moves.append(&mut self.project(game, 0, -1));
        moves
    }

    fn bishop_moves(&self, game: &Board) -> Vec<Move> {
        let mut moves = self.project(game, 1, 1);
        moves.append(&mut self.project(game, -1, 1));
        moves.append(&mut self.project(game, -1, -1));
        moves.append(&mut self.project(game, 1, -1));
        moves
    }

    //project in a direction until a collision or end of board
    fn project(&self, game: &Board, rise: i32, run: i32) -> Vec<Move> {
        let mut moves = Vec::new();

        let mut coord = self.location;

        //loop until end of board
        loop {
            //increment with row and col char
            //involves typecasting to ensure no overflows
            if rise < 0 {
                if coord.row == 0 {
                    break; //can't subtract from 0
                }
                coord.row -= (-1 * rise) as usize;
            } else {
                coord.row += rise as usize
            };

            if run < 0 {
                if coord.col == 0 {
                    break; //can't subtract from 0
                }
                coord.col -= (-1 * run) as usize;
            } else {
                coord.col += run as usize;
            }

            if coord.col > 7 {
                break;
            }

            if !coord.is_valid() {
                break;
            }

            let square = game.square(&coord);

            //create move
            let m = Move::new(self.location, coord, game, None, false);

            if square.piece == None {
                moves.push(m)
            } else if let Some(piece) = square.piece {
                if piece.color != self.color {
                    //capture?
                    moves.push(m)
                }

                break; //it blocks movement regardless
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    //used to create a test board
    fn default_board() -> Board {
        let mut b = Board::new();
        b.populate_starting_pos();
        b
    }

    #[test]
    fn undo_move() {
        //create default board
        let mut b = default_board();
        let backup = b.clone();

        //move king to center of board because why not
        let mut m = Move::new(
            Coordinate::new(4, 0),
            Coordinate::new(4, 3),
            &b,
            None,
            false,
        );
        m.perform_on(&mut b);
        m.undo_on(&mut b);

        assert_eq!(b, backup);
    }

    #[test]
    fn no_rook_moves() {
        //no rook moves at the start
        let b = default_board();

        let c = Coordinate::new(0, 7);
        let square = b.square(&c);
        let piece = square.piece.unwrap();
        assert_eq!(piece.potential_moves(&b).len(), 0);
    }
}
