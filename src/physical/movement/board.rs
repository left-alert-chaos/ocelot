//!# board
//!This module holds a simple impl block to add check logic to the Board object defined at
//!physical::board::Board.

use crate::physical::board::{self, Board};
use crate::physical::movement::{Action, MoveInfo};

impl Board {
    pub fn is_check(&self, player: board::Color) -> bool {
        let mut kings = self.pieces();
        kings.retain(|x| x.color == player && x.ptype == board::PieceType::King);

        if kings.len() != 1 {
            eprintln!(
                "Board::is_check({player}): Instead of 1, kings.len() is {}. Returning false.\nKings: {kings:?}\nPosition:\n{}",
                kings.len(),
                self.draw(),
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
            board::Color::White => self.move_info.white_potential_moves.is_empty(),
            board::Color::Black => self.move_info.black_potential_moves.is_empty(),
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

        let mut captures = Vec::new();
        let mut normal = Vec::new();

        //crude move ordering
        for action in moves {
            let value = action.evaluate();
            if value < 1.0 || value == 2.0 {
                normal.push(action);
            } else {
                captures.push(action);
            }
        }

        captures.append(&mut normal);

        captures
    }

    pub fn black_potential_moves(&self) -> Vec<Box<dyn Action>> {
        let mut moves = Vec::new();

        for piece in self.black_pieces() {
            moves.append(&mut piece.potential_moves(self));
        }

        let mut captures = Vec::new();
        let mut normal = Vec::new();

        //crude move ordering
        for action in moves {
            let value = action.evaluate();
            if value < 1.0 || value == 2.0 {
                normal.push(action);
            } else {
                captures.push(action);
            }
        }

        captures.append(&mut normal);

        captures
    }
}
