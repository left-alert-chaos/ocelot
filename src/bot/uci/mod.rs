//!# uci
//!This module holds code to generate UCI from engine objects and generate engine objects from UCI.

mod fen;
use crate::physical::*;

pub trait ToUCI {
    fn generate(&self) -> String;
}

impl ToUCI for Move {
    fn generate(&self) -> String {
        let mut repr = format!("{}{}", self.from, self.to);

        if let Some(promotion) = self.promotion {
            repr.push(promotion.letter());
        }

        repr
    }
}

impl ToUCI for Castle {
    fn generate(&self) -> String {
        let row = self.player.home_rank() + 1;
        //get target column by converting column index to column letter
        let target = board::LETTERS.chars().nth(self.side.king_end_col()).unwrap();

        format!("e{row}{target}{row}")
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //generate an impossible pawn move to test all generation
    fn generate_move() {
        let b: Board = Default::default();
        let expected = String::from("e2e4q");
        let m = Move::new(Coordinate::new(4, 1), Coordinate::new(4, 3), &b, Some(board::PieceType::Queen), false);
        let generated = m.generate();
        assert_eq!(expected, generated);
    }

    #[test]
    fn generate_castle() {
        let expected = String::from("e8g8");
        let generated = Castle::new(CastleSide::KingSide, board::Color::Black).generate();
        assert_eq!(generated, expected);
    }
}
