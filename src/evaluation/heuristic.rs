//!# static
//!This module holds code to do static evaluation of a board.
//!Most of it is in an impl block for crate::physical::board::Board.

use crate::physical::*;

impl Board {
    pub fn evaluation(&mut self) -> f64 {
        if self.is_checkmate(board::Color::White) {
            return f64::NEG_INFINITY; //black winning
        } else if self.is_checkmate(board::Color::Black) {
            return f64::INFINITY; //white winning
        }

        //base heuristic is just white material minus black material
        let mut valuation = self.white_material() as f64;
        valuation -= self.black_material() as f64;

        //do various heuristic checks
        valuation += self.pawns_in_center();
        valuation += self.can_castle();
        valuation += self.has_castled();
        valuation += self.queen_movement();

        valuation
    }

    fn white_material(&self) -> i32 {
        let pieces = self.white_pieces();
        let mut value = 0;
        for piece in pieces {
            value += piece.ptype.value();
        }
        value
    }

    fn black_material(&self) -> i32 {
        let pieces = self.black_pieces();
        let mut value = 0;
        for piece in pieces {
            value += piece.ptype.value();
        }
        value
    }

    //check if there are pawns in the center, and give a score for it.
    fn pawns_in_center(&self) -> f64 {
        let mut value = 0.0;

        for col in 3..=4 {
            for row in 3..=4 {
                //get piece
                let location = Coordinate::new(col as usize, row as usize);
                let square = self.square(&location);
                let Some(piece) = square.piece else {
                    continue;
                };

                //determine score
                if piece.ptype.value() == 1 {
                    value += 1.0 * (piece.color.value() as f64);
                }
            }
        }

        value
    }

    //see which castles are legal
    fn can_castle(&mut self) -> f64 {
        let mut value = 0.0;

        let mut white_queenside = Castle::new(CastleSide::QueenSide, board::Color::White);
        let mut black_queenside = Castle::new(CastleSide::QueenSide, board::Color::Black);
        let mut white_kingside = Castle::new(CastleSide::KingSide, board::Color::White);
        let mut black_kingside = Castle::new(CastleSide::KingSide, board::Color::Black);

        if !white_queenside.is_illegal(self) {
            value += 0.8;
        }
        if !black_queenside.is_illegal(self) {
            value -= 0.8;
        }
        if !white_kingside.is_illegal(self) {
            value += 0.8;
        }
        if !black_kingside.is_illegal(self) {
            value -= 0.8;
        }

        value
    }

    fn has_castled(&self) -> f64 {
        let mut value = 0.0;
        if self.white_castled {
            value += 1.0;
        }
        if self.black_castled {
            value -= 1.0;
        }
        value
    }

    //reward queen staying on home square
    fn queen_movement(&self) -> f64 {
        let mut value = 0.0;
        let black_queen_square = self.square(&Coordinate::new(3, 7));
        let white_queen_square = self.square(&Coordinate::new(3, 0));

        if let Some(piece) = black_queen_square.piece
            && piece.color == board::Color::Black
            && piece.ptype == board::PieceType::Queen
        {
            value -= 0.5;
        }

        if let Some(piece) = white_queen_square.piece
            && piece.color == board::Color::White
            && piece.ptype == board::PieceType::Queen
        {
            value += 0.5;
        }

        value
    }
}
