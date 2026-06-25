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
        Ok(Self::new(from, to, &current_board.unwrap(), promotion, true))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_move() {
        let b: Board = Default::default();
        let expected = Move::new(Coordinate::new(4, 1), Coordinate::new(4, 3), &b, None, false);
        let generated = Move::parse(String::from("e2e4"), Some(&b));
        assert_eq!(generated.unwrap(), expected);
    }
}
