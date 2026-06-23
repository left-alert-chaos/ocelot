//!# board
//!This module holds a simple impl block to add check logic to the Board object defined at
//!physical::board::Board.

use crate::physical::board::{self, Board};
use crate::physical::movement::{MoveInfo, Action};

impl Board {
    pub fn is_check(&self, player: board::Color) -> bool {
        let mut kings = self.pieces();
        kings.retain(|x| x.color == player && x.ptype == board::PieceType::King);

        if kings.len() != 1 {
            eprintln!(
                "Board::is_check(): Instead of 1, kings.len() is {}. Returning false.",
                kings.len()
            );
            false
        } else {
            //find if location is in player's threats
            if player == board::Color::White {
                self.move_info
                    .black_threatened_squares
                    .contains(&kings[0].location)
            } else {
                self.move_info
                    .white_threatened_squares
                    .contains(&kings[0].location)
            }
        }
    }

    ///# is_checkmate
    ///Assumes that move_info.*potential_moves has been filtered for legality.
    pub fn is_checkmate(&self, player: board::Color) -> bool {
        if !self.is_check(player) {
            return false;
        }

        match player {
            board::Color::White => {
                self.move_info.white_potential_moves.is_empty()
            }
            board::Color::Black => {
                self.move_info.black_potential_moves.is_empty()
            }
        }
    }

    pub fn update(&mut self) {
        self.move_info = MoveInfo::from(self);
    }

    pub fn white_potential_moves(&self) -> Vec<Box<dyn Action>> {
        let mut moves = Vec::new();

        for piece in self.white_pieces() {
            moves.append(&mut piece.potential_moves(self));
        }

        moves
    }

    pub fn black_potential_moves(&self) -> Vec<Box<dyn Action>> {
        let mut moves = Vec::new();

        for piece in self.black_pieces() {
            moves.append(&mut piece.potential_moves(self));
        }

        moves
    }
}
