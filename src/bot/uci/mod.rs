//!# uci
//!This module holds code to generate UCI from engine objects and generate engine objects from UCI.

mod fen;
#[allow(dead_code)]
mod go;
use crate::physical::*;

pub trait ToUCI {
    fn generate(&self) -> String;
}

pub trait FromUCI {
    fn parse(representation: String, current_board: &Board) -> Result<Box<dyn Action>, ()>;
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
    fn parse(mut representation: String, current_board: &Board) -> Result<Box<dyn Action>, ()> {
        let backup_repr = representation.clone();
        let current_board = current_board.duplicate();
        if representation.len() > 5 || representation.len() < 4 || representation.contains(" ") {
            return Err(());
        }

        let from = Coordinate::from(&mut representation)?;
        if !from.is_valid() {
            return Err(());
        }

        let to = Coordinate::from(&mut representation)?;
        if !to.is_valid() {
            return Err(());
        }

        //weirdly enough, this prevents illegal castling having side effects
        if let Some(captured_piece) = current_board.square(&to).piece {
            if let Some(moving_piece) = current_board.square(&from).piece {
                if captured_piece.color == moving_piece.color {
                    return Err(());
                }
            } else {
                eprintln!(
                    "Generating a move from UCI {backup_repr}, but there isn't a piece where the move is from."
                );
            }
        }

        let promotion = if !representation.is_empty() {
            board::PieceType::from(representation.remove(0)).ok()
        } else {
            None
        };

        //If this breaks, look at the if en_passant block in new()
        //assume en passant and if it isn't legal catch it in constructor
        Ok(Box::new(Self::new(
            from,
            to,
            &current_board,
            promotion,
            true,
        )))
    }
}

#[allow(refining_impl_trait)]
impl FromUCI for Castle {
    fn parse(mut representation: String, current_board: &Board) -> Result<Box<dyn Action>, ()> {
        let mut current_board = current_board.duplicate();
        if representation.chars().nth(0).unwrap() != 'e'
            || representation.len() != 4
            || representation.contains(" ")
        {
            return Err(());
        }

        let king_loc = Coordinate::from(&mut representation)?;
        if !king_loc.is_valid() {
            return Err(());
        }
        let king_target = Coordinate::from(&mut representation)?;
        if !king_target.is_valid() {
            return Err(());
        }

        let player = match king_loc.row {
            0 => board::Color::White,
            7 => board::Color::Black,
            _ => {
                return Err(());
            }
        };

        let side = match king_target.col {
            2 => CastleSide::QueenSide,
            6 => CastleSide::KingSide,
            _ => return Err(()),
        };

        let mut castle = Self::new(side, player);
        if castle.is_illegal(&mut current_board) {
            return Err(());
        }

        Ok(Box::new(castle))
    }
}

impl ToUCI for Castle {
    fn generate(&self) -> String {
        let row = self.player.home_rank() + 1;
        //get target column by converting column index to column letter
        let target = board::LETTERS
            .chars()
            .nth(self.side.king_end_col())
            .unwrap();

        format!("e{row}{target}{row}")
    }
}

pub fn parse_action(representation: String, current_board: &mut Board) -> Box<dyn Action> {
    //try generating a castle. If that doesn't work, make a Move
    let castle = Castle::parse(representation.clone(), current_board);
    if let Ok(c) = castle {
        return c;
    }

    //otherwise, create a move

    let m = Move::parse(representation, current_board).unwrap();
    m
}

///Like parse_action, but doesn't call unwrap()
pub fn safe_parse_action(
    representation: String,
    current_board: &mut Board,
) -> Result<Box<dyn Action>, ()> {
    if let Ok(c) = Castle::parse(representation.clone(), current_board) {
        return Ok(c);
    }

    if let Ok(m) = Move::parse(representation, current_board) {
        return Ok(m);
    }

    Err(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //generate an impossible pawn move to test all generation
    fn generate_move() {
        let b: Board = Default::default();
        let expected = String::from("e2e4q");
        let m = Move::new(
            Coordinate::new(4, 1),
            Coordinate::new(4, 3),
            &b,
            Some(board::PieceType::Queen),
            false,
        );
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
