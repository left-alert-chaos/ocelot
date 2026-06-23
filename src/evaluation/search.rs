//!# search
//!This module holds logic for looking into the future.

use crate::physical::*;
use std::collections::HashMap;

pub struct SearchTree {
    player: board::Color,
    positions: i32,
    root: SearchNode,
    depth: i32,
}

impl SearchTree {
    pub fn new(game: &Board, depth: i32) -> Self {
        let game = game.duplicate();

        Self {
            player: game.turn,
            positions: 0,
            root: SearchNode::new(game),
            depth,
        }
    }

    pub fn best_move(mut self) -> Result<Box<dyn Action>, ()> {
        self.root.alphabeta(self.depth, f64::NEG_INFINITY, f64::INFINITY, true, true);
        
        //convert option to result
        match self.root.best_move {
            Some(best_move) => Ok(best_move),
            None => Err(()),
        }
    }
}

pub struct SearchNode {
    board: Board,
    value: f64,
    best_move: Option<Box<dyn Action>>,
}

impl SearchNode {
    fn new(game: Board) -> Self {
        let value = game.evaluation();
        Self {
            board: game,
            value,
            best_move: None,
        }
    }

    fn alphabeta(&mut self, depth: i32, mut alpha: f64, mut beta: f64, maximizing_player: bool, is_root: bool) -> f64 {
        //not the best solution, but it works
        let mut test_board = self.board.duplicate();

        //end of game or tree
        if depth == 0 || self.value == f64::INFINITY || self.value == f64::NEG_INFINITY {
            return self.value * if maximizing_player {1.0} else {-1.0};
        }

        //get potential moves
        let potential_moves = if self.board.turn == board::Color::White {&self.board.move_info.white_potential_moves} else {&self.board.move_info.black_potential_moves};

        let mut value: f64;

        //create child, play move, get value
        if maximizing_player {
            value = f64::NEG_INFINITY;

            let mut best_value: Option<f64> = None;
            for m in potential_moves {
                let mut action = m.duplicate();

                if action.is_illegal(&mut test_board) {
                    continue;
                }

                //create child
                let mut child = Self::new(self.board.duplicate());
                action.perform_on(&mut child.board);

                //recursion and pruning
                let recursion_result = child.alphabeta(depth - 1, alpha, beta, false, false);
                value = max(value, recursion_result);

                //handle root node best move logic
                if is_root {
                    if self.best_move.is_none() {
                        self.best_move = Some(action);
                        best_value = Some(child.value);
                        continue;
                    }

                    //otherwise, check for better
                    let unwrapped_best_value = best_value.unwrap();
                    if child.value >= unwrapped_best_value {
                        best_value = Some(child.value);
                        self.best_move = Some(action);
                    }
                }

                if value >= beta {
                    break;
                }
                alpha = max(alpha, value);
            }


        } else {
            value = f64::INFINITY;
            for m in potential_moves {
                let mut action = m.duplicate();

                if action.is_illegal(&mut test_board) {
                    continue;
                }

                //create child
                let mut child = Self::new(self.board.duplicate());
                action.perform_on(&mut child.board);

                //recursion and pruning
                value = min(value, child.alphabeta(depth - 1, alpha, beta, true, false));
                if value <= alpha {
                    break;
                }
                beta = min(beta, value);
            }
        }

        self.value = value;
        value
    }
}

fn max(a: f64, b: f64) -> f64 {
    if a > b {
        a
    } else {
        b
    }
}

fn min(a: f64, b: f64) -> f64 {
    if a < b {
        a
    } else {
        b
    }
}
