//!# movement
//!Handles impls for piece movement, move generation, legality checking, etc.

use crate::physical::board;
use std::fmt;

pub trait Action {
    fn perform_on(&self, game: &mut board::Board);
    //fn undo_on(&self, game: &mut board::Board);
    fn is_illegal(&self, _game: &board::Board) -> bool {
        false
    }
}

///# Move
///Holds information about a move and implements Action, so it can be performed.
///
///# Public Methods
///
///## new(from: board::Coordinate, to: board::Coordinate, en_passant: Option<board::Coordinate>,
///value: u32, promotion: Option<board::PieceType>)
#[derive(Copy, Clone)]
pub struct Move {
    from: board::Coordinate,
    to: board::Coordinate,
    en_passant: Option<board::Coordinate>,
    value: u32,
    promotion: Option<board::PieceType>,
}

impl fmt::Display for Move {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Move from {} to {} with value {}", self.from, self.to, self.value)
    }
}

impl Action for Move {
    fn perform_on(&self, game: &mut board::Board) {
        let from_square = game.mut_square(&self.from);
        let mut moving_piece = from_square.piece.expect(format!("{self} isn't legal because it is from a square without a piece.").as_str());

        //handle promotion
        if let Some(promo_type) = self.promotion {
            moving_piece.ptype = promo_type;
        }

        //perform move
        game.remove_piece_on(&self.from);
        game.put_piece_on(&self.to, moving_piece);
        game.locations.push(self.to);

        //lastly, delete the piece on the en_passant square
        if let Some(ep_loc) = self.en_passant {
            game.remove_piece_on(&ep_loc);
        }
    }
}

impl Move {
    pub fn new(from: board::Coordinate, to: board::Coordinate, game: &board::Board, promotion: Option<board::PieceType>, en_passant: bool) -> Self {
        let en_passant_location = if en_passant {
            //direction of en passant, NOT pawn movement direction.
            let direction: i32 = if from.row < to.row {-1} else {1};
            Some(board::Coordinate::new(to.col, to.row + direction as usize))
        } else {None};

        //do a quick-and-dirty evaluation. This is overwritten by search, but useful in some places.
        let value = if let Some(ptype) = promotion {
            ptype.value() - 1
        } else {
            //if there is a piece, get the value of that piece. otherwise, no value.
            let to_square = game.square(&to);
            if let Some(piece) = to_square.piece {
                piece.ptype.value()
            } else {0}
        };

        Move {
            from,
            to,
            en_passant: en_passant_location,
            value: value as u32,
            promotion,
        }
    }
}
