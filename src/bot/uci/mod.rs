//!# uci
//!This module holds code to generate UCI from engine objects and generate engine objects from UCI.

mod fen;
use crate::physical::*;

pub trait FromUCI {
    fn parse(representation: String, current_board: Option<&Board>) -> Result<impl FromUCI, ()>;
}

pub trait ToUCI {
    fn generate(&self) -> String;
}

#[allow(refining_impl_trait)]
impl FromUCI for Move {
    fn parse(mut representation: String, current_board: Option<&Board>) -> Result<Self, ()> {
        if representation.len() > 5 || representation.len() < 4 {
            return Err(())
        }

        let from = Coordinate::from(&mut representation);

        let to = Coordinate::from(&mut representation);

        let promotion = if !representation.is_empty() {
            board::PieceType::from(representation.remove(0)).ok()
        } else {
            None
        };

        //If this breaks, look at the if en_passant block in new()
        //assume en passant and if it isn't legal catch it in constructor
        Ok(Self::new(from, to, current_board.unwrap(), promotion, true))
    }
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

#[allow(refining_impl_trait)]
impl FromUCI for Castle {
    fn parse(mut representation: String, _current_board: Option<&Board>) -> Result<Self, ()> {
        if representation.len() != 4 {
            return Err(())
        }

        let king_loc = Coordinate::from(&mut representation);
        let king_target = Coordinate::from(&mut representation);
        
        let player = match king_loc.row {
            0 => board::Color::White,
            7 => board::Color::Black,
            _ => {
                return Err(())
            }
        };

        let side = match king_target.col {
            2 => CastleSide::QueenSide,
            6 => CastleSide::KingSide,
            _ => {
                return Err(())
            }
        };

        Ok(Self::new(side, player))
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
    //create e4, the subjectively objectively correct opening move.
    fn parse_move() {
        let b: Board = Default::default();
        let expected = Move::new(Coordinate::new(4, 1), Coordinate::new(4, 3), &b, None, false);
        let generated = Move::parse(String::from("e2e4"), Some(&b));
        assert_eq!(generated.unwrap(), expected);
    }

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
    fn parse_castle() {
        let expected = Castle::new(CastleSide::QueenSide, board::Color::Black);
        assert_eq!(Castle::parse(String::from("e8c8"), None).unwrap(), expected);
    }

    #[test]
    fn generate_castle() {
        let expected = String::from("e8g8");
        let generated = Castle::new(CastleSide::KingSide, board::Color::Black).generate();
        assert_eq!(generated, expected);
    }
}
