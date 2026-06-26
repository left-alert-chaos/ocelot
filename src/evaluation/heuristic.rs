//!# static
//!This module holds code to do static evaluation of a board.
//!Most of it is in an impl block for crate::physical::board::Board.

use crate::physical::*;

impl Board {
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
    pub fn pawns_in_center(&self) -> f64 {
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
                    value += 0.5 * (piece.color.value() as f64);
                }
            }
        }

        value
    }

    pub fn evaluation(&self) -> f64 {
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

        valuation
    }
}
