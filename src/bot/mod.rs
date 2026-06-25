//!# bot
//!This module holds code to wrap search and eval and move gen into an accessible API. It also holds
//!the UCI implementation.

mod uci;
use crate::physical::*;
use crate::evaluation::SearchTree;

///# Ocelot
///The whole reason we're here. This struct keeps track of game state and uses SearchTrees to
///generate the best move for the player the bot is assigned.
pub struct Ocelot {
    pub(crate) board: Board,
    player: board::Color,
    depth: i32,
}

impl Ocelot {
    pub fn new(board: &Board, player: board::Color, depth: i32) -> Self {
        Self {
            board: board.duplicate(),
            player,
            depth,
        }
    }

    fn best_move(&self) -> Result<Box<dyn Action>, ()> {
        let mut tree = SearchTree::new(&self.board, self.depth);
        tree.best_move()
    }


    ///This tries to get the best move with a SearchTree. If it can't find one, it returns the first
    ///legal one.
    ///Remember that this doesn't apply the move, just generates it.
    pub fn safe_best_move(&mut self) -> Box<dyn Action> {
        let search_result = self.best_move();
        if search_result.is_ok() {
            return search_result.unwrap();
        }

        //if that didn't work, just get the first legal move
        eprintln!("Ocelot::safe_best_move(): Search didn't return a move, so getting first legal one.");
        let potential_moves = match self.player {
            board::Color::White => self.board.white_potential_moves(),
            board::Color::Black => self.board.black_potential_moves(),
        };

        //look for legal moves
        for mut m in potential_moves {
            if !m.is_illegal(&mut self.board) {
                return m
            }
        }

        //not much else you can do
        panic!("Ocelot::safe_best_move(): Search didn't return a move and no potential moves are legal!");
    }

    pub fn perform_on_self(&mut self, mut action: Box<dyn Action>) {
        action.perform_on(&mut self.board);
    }
}
