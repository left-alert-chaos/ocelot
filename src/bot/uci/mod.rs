//!# uci
//!This module holds code to generate UCI from engine objects and generate engine objects from UCI.

mod fen;
use crate::physical::*;

pub trait ToUCI {
    fn generate(&self) -> String;
}

pub trait FromUCI {
    fn parse(representation: String, current_board: &mut Board) -> Result<Box<dyn Action>, ()>;
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
impl FromUCI for Move {
    fn parse(mut representation: String, current_board: &mut Board) -> Result<Box<dyn Action>, ()> {
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
        Ok(Box::new(Self::new(from, to, current_board, promotion, true)))
    }
}

#[allow(refining_impl_trait)]
impl FromUCI for Castle {
    fn parse(mut representation: String, current_board: &mut Board) -> Result<Box<dyn Action>, ()> {
        if representation.len() != 4 {
            return Err(())
        }

        if representation.chars().nth(0).unwrap() != 'e' {
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

        let mut castle = Self::new(side, player);
        if castle.is_illegal(current_board) {
            return Err(())
        }

        Ok(Box::new(castle))
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

pub fn parse_action(representation: String, current_board: &mut Board) -> Box<dyn Action> {
    //try generating a castle. If that doesn't work, make a Move
    let castle = Castle::parse(representation.clone(), current_board);
    if let Ok(c) = castle {
        println!("uci::parse_action(): Returning caslte {c}");
        return c;
    }

    //otherwise, create a move
    
    let m = Move::parse(representation, current_board).unwrap();
    println!("uci::parse_action(): Returing move {m}");
    m
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
