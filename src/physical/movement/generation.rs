//!# generation
//!This module holds logic for generating moves. The logic is all in an impl block for
//!crate::physical::board::Piece.

use crate::physical::board::{self, Coordinate, Piece, Board};
use crate::physical::movement::{Action, Move, Castle, types::CastleSide};

//seperate impl for move generation
#[allow(dead_code)]
impl Piece {
    pub fn potential_moves(&self, game: &Board) -> Vec<Box<dyn Action>> {
        match self.ptype {
            board::PieceType::Rook => self.rook_moves(game),
            board::PieceType::Knight => self.knight_moves(game),
            board::PieceType::Bishop => self.bishop_moves(game),
            board::PieceType::Queen => {
                //queen movement is just rook and bishop combined
                let mut moves = self.rook_moves(game);
                moves.append(&mut self.bishop_moves(game));
                moves
            }
            board::PieceType::King => self.king_moves(game),
            board::PieceType::Pawn(_) => self.pawn_moves(game),
        }
    }

    fn rook_moves(&self, game: &Board) -> Vec<Box<dyn Action>> {
        let mut moves = self.project(game, 1, 0);
        moves.append(&mut self.project(game, -1, 0));
        moves.append(&mut self.project(game, 0, 1));
        moves.append(&mut self.project(game, 0, -1));
        moves
    }

    fn bishop_moves(&self, game: &Board) -> Vec<Box<dyn Action>> {
        let mut moves = self.project(game, 1, 1);
        moves.append(&mut self.project(game, -1, 1));
        moves.append(&mut self.project(game, -1, -1));
        moves.append(&mut self.project(game, 1, -1));
        moves
    }

    fn king_moves(&self, game: &Board) -> Vec<Box<dyn Action>> {
        let mut moves: Vec<Box<dyn Action>> = Vec::new();

        //castling legality is determined later; for now it's just included
        moves.push(Box::new(Castle::new(CastleSide::KingSide, self.color)));
        moves.push(Box::new(Castle::new(CastleSide::QueenSide, self.color)));

        //try all 8 surrounding squares
        self.try_square(game, 1, 1, &mut moves);
        self.try_square(game, 0, 1, &mut moves);
        self.try_square(game, -1, 1, &mut moves);
        self.try_square(game, -1, 0, &mut moves);
        self.try_square(game, -1, -1, &mut moves);
        self.try_square(game, 0, -1, &mut moves);
        self.try_square(game, 1, -1, &mut moves);
        self.try_square(game, 1, 0, &mut moves);

        moves
    }

    fn knight_moves(&self, game: &Board) -> Vec<Box<dyn Action>> {
        let mut moves: Vec<Box<dyn Action>> = Vec::new();

        self.try_square(game, 2, 1, &mut moves);
        self.try_square(game, 1, 2, &mut moves);
        self.try_square(game, -1, 2, &mut moves);
        self.try_square(game, -2, 1, &mut moves);
        self.try_square(game, -2, -1, &mut moves);
        self.try_square(game, -1, -2, &mut moves);
        self.try_square(game, 1, -2, &mut moves);
        self.try_square(game, 2, -1, &mut moves);

        moves
    }

    fn pawn_moves(&self, game: &Board) -> Vec<Box<dyn Action>> {
        let mut moves: Vec<Box<dyn Action>> = Vec::new();
        //multiplier for movement
        let direction = self.color.value();

        //move one square
        let one_square_movement = self.one_square_pawn_movement(game, direction);
        if let Some(mut pushes) = one_square_movement {
            moves.append(&mut pushes);

            //could move one, so can move 2
            if !self.has_moved
                && let Some(m) = self.two_square_pawn_movement(game, direction)
            {
                moves.push(Box::new(m));
            }
        }

        self.pawn_capture(direction, 1, game, &mut moves);
        self.pawn_capture(direction, -1, game, &mut moves);

        self.try_en_passant(direction, 1, game, &mut moves);
        self.try_en_passant(direction, -1, game, &mut moves);

        moves
    }

    fn try_en_passant(&self, rise: i32, run: i32, game: &Board, buffer: &mut Vec<Box<dyn Action>>) {
        let neighbor_loc = match self.location.with_offset(0, run) {
            Ok(loc) => loc,
            Err(_) => return,
        };

        //get piece at loc
        let piece = match game.square(&neighbor_loc).piece {
            Some(piece) => piece,
            None => return,
        };

        //determine if en passantable
        if piece.ptype == board::PieceType::Pawn(game.round) && piece.color != self.color {
            //possible!
            let en_passant_location = match self.location.with_offset(rise, run) {
                Ok(loc) => loc,
                Err(_) => return,
            };
            buffer.push(Box::new(Move::new(
                self.location,
                en_passant_location,
                game,
                None,
                true,
            )));
        }
    }

    fn pawn_capture(&self, rise: i32, run: i32, game: &Board, buffer: &mut Vec<Box<dyn Action>>) {
        let maybe_loc = self.location.with_offset(rise, run);
        if maybe_loc.is_err() {
            return;
        }
        let loc = maybe_loc.unwrap();

        if loc.row == self.pawn_target_rank() {
            //all promotion types
            self.only_captures(game, &loc, Some(board::PieceType::Queen), buffer);
            self.only_captures(game, &loc, Some(board::PieceType::Rook), buffer);
            self.only_captures(game, &loc, Some(board::PieceType::Bishop), buffer);
            self.only_captures(game, &loc, Some(board::PieceType::Knight), buffer);
        } else {
            //only capture
            self.only_captures(game, &loc, None, buffer);
        }
    }

    fn one_square_pawn_movement(
        &self,
        game: &Board,
        direction: i32,
    ) -> Option<Vec<Box<dyn Action>>> {
        let maybe_location = self.location.with_offset(direction, 0);
        if maybe_location.is_err() {
            return None;
        }

        let location = maybe_location.unwrap();

        let value = self.square_value(game, &location);
        if value != Some(0) {
            return None;
        }

        let mut moves: Vec<Box<dyn Action>> = Vec::new();

        if location.row == self.pawn_target_rank() {
            //go through all promotion types
            moves.push(Box::new(Move::new(
                self.location,
                location,
                game,
                Some(board::PieceType::Queen),
                false,
            )));
            moves.push(Box::new(Move::new(
                self.location,
                location,
                game,
                Some(board::PieceType::Rook),
                false,
            )));
            moves.push(Box::new(Move::new(
                self.location,
                location,
                game,
                Some(board::PieceType::Bishop),
                false,
            )));
            moves.push(Box::new(Move::new(
                self.location,
                location,
                game,
                Some(board::PieceType::Knight),
                false,
            )));
        } else {
            moves.push(Box::new(Move::new(
                self.location,
                location,
                game,
                None,
                false,
            )));
        }

        Some(moves)
    }

    //find two-square move without checking for legality/blocked
    fn two_square_pawn_movement(&self, game: &Board, direction: i32) -> Option<Move> {
        let location = match self.location.with_offset(direction * 2, 0) {
            Ok(loc) => loc,
            Err(_) => return None,
        };

        if self.square_value(game, &location) == Some(0) {
            return Some(Move::new(self.location, location, game, None, false));
        }

        None
    }

    fn pawn_target_rank(&self) -> usize {
        match self.color {
            board::Color::White => 7,
            board::Color::Black => 0,
        }
    }

    //like try_square(), but only works if there's a capture
    fn only_captures(
        &self,
        game: &Board,
        location: &Coordinate,
        promotion: Option<board::PieceType>,
        buffer: &mut Vec<Box<dyn Action>>,
    ) {
        let value = self.square_value(game, location);
        if value.is_some_and(|x| x != 0) {
            buffer.push(Box::new(Move::new(
                self.location,
                *location,
                game,
                promotion,
                false,
            )));
        }
    }

    fn try_square(&self, game: &Board, rise: i32, run: i32, buffer: &mut Vec<Box<dyn Action>>) {
        //apply movement
        let location = match self.location.with_offset(rise, run) {
            Ok(loc) => loc,
            Err(_) => return,
        };

        //find square value
        let value = self.square_value(game, &location);
        if value.is_some() {
            //create move to square
            buffer.push(Box::new(Move::new(
                self.location,
                location,
                game,
                None,
                false,
            )));
        }
    }

    //project in a direction until a collision or end of board
    fn project(&self, game: &Board, rise: i32, run: i32) -> Vec<Box<dyn Action>> {
        let mut moves: Vec<Box<dyn Action>> = Vec::new();

        let mut coord = self.location;

        //loop until end of board
        loop {
            //increment with row and col char
            //involves typecasting to ensure no overflows
            if rise < 0 {
                if coord.row == 0 {
                    break; //can't subtract from 0
                }
                coord.row -= -rise as usize;
            } else {
                coord.row += rise as usize
            };

            if run < 0 {
                if coord.col == 0 {
                    break; //can't subtract from 0
                }
                coord.col -= -run as usize;
            } else {
                coord.col += run as usize;
            }

            if coord.col > 7 {
                break;
            }

            if !coord.is_valid() {
                break;
            }

            //create move
            let m = Move::new(self.location, coord, game, None, false);

            let value = self.square_value(game, &coord);
            if let Some(v) = value {
                moves.push(Box::new(m));
                if v > 0 {
                    break;
                }
            } else {
                break; //returned None, so illegal
            }
        }

        moves
    }

    //returns Some(0) for empty square and None for square occupied by piece of same color
    fn square_value(&self, game: &Board, location: &Coordinate) -> Option<u32> {
        let square = game.square(location);
        if let Some(piece) = square.piece {
            if piece.color != self.color {
                Some(piece.ptype.value() as u32)
            } else {
                None
            }
        } else {
            Some(0)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    //returns Some(0) for empty square and None for square occupied by piece of same color
    fn square_value() {
        let b: Board = Default::default();
        let p_loc = Coordinate::new(0, 0); //left white rook
        let enemy_rook_loc = Coordinate::new(0, 7);
        let enemy_pawn_loc = Coordinate::new(0, 6);
        let friendly_pawn_loc = Coordinate::new(0, 1);
        let square = b.square(&p_loc);
        let piece = square.piece.unwrap();
        assert_eq!(piece.square_value(&b, &enemy_rook_loc), Some(5));
        assert_eq!(piece.square_value(&b, &enemy_pawn_loc), Some(1));
        assert_eq!(piece.square_value(&b, &friendly_pawn_loc), None);
    }
}
