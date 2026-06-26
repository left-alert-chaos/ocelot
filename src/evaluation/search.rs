//!# search
//!This module holds logic for looblack_king into the future.

use crate::physical::*;
use std::fmt;

pub struct SearchTree {
    pub(crate) root: SearchNode,
    depth: i32,
}

impl SearchTree {
    pub fn new(game: &Board, depth: i32) -> Self {
        let game = game.duplicate();
        let turn = game.turn;

        Self {
            root: SearchNode::new(game, turn),
            depth,
        }
    } 

    pub fn safe_best_move(&mut self) -> Box<dyn Action> {
        let search_result = self.best_move();
        if let Ok(action) = search_result {
            return action;
        }

        //otherwise, find best legal move
        let mut board = self.root.board.duplicate();
        let mut potential_moves = if self.root.player == board::Color::White {board.white_potential_moves()} else {board.black_potential_moves()};

        //look at all legal moves
        let mut best_value = potential_moves[0].evaluate();
        let mut best_action = potential_moves[0].duplicate();
        let mut value;
        for action in potential_moves.iter_mut() {
            if action.is_illegal(&mut board) {
                continue;
            }

            value = action.evaluate();
            if value > best_value {
                best_value = value;
                best_action = action.duplicate();
            }
        }

        best_action.duplicate()
    }

    fn best_move(&mut self) -> Result<Box<dyn Action>, ()> {
        self.root.alphabeta(self.depth, f64::NEG_INFINITY, f64::INFINITY, true);
        
        //convert option to result
        match &self.root.best_move {
            Some(best_move) => Ok(best_move.duplicate()),
            None => Err(()),
        }
    }
}

pub struct SearchNode {
    board: Board,
    value: f64,
    pub(crate) best_move: Option<Box<dyn Action>>,
    pub(crate) best_child: Box<Option<SearchNode>>,
    player: board::Color,
}

impl SearchNode {
    fn new(game: Board, player: board::Color) -> Self {
        let value = game.evaluation();
        Self {
            board: game,
            value,
            best_move: None,
            best_child: Box::new(None),
            player,
        }
    }

    fn alphabeta(&mut self, depth: i32, mut alpha: f64, mut beta: f64, maximizing_player: bool) -> f64 {
        //not the best solution, but it works
        let mut test_board = self.board.duplicate();

        //end of game or tree
        if depth == 0 || self.value == f64::INFINITY || self.value == f64::NEG_INFINITY {
            let result = self.value * (self.player.value() as f64);
            return result;
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
                let mut child_board = self.board.duplicate();
                action.perform_on(&mut child_board);
                let mut child = Self::new(child_board, self.player);

                //recursion
                let recursion_result = child.alphabeta(depth - 1, alpha, beta, false);
                value = max(value, recursion_result);

                //handle best move logic
                
                //prepopulate if no best params yet
                if self.best_move.is_none() {
                    self.best_move = Some(action);
                    best_value = Some(child.evaluate());

                    //put the child in a box haha
                    *self.best_child = Some(child);
                    continue;
                }

                //otherwise, check for better
                child.value = child.evaluate();
                let unwrapped_best_value = best_value.unwrap();
                if child.value >= unwrapped_best_value {
                    best_value = Some(child.value);
                    self.best_move = Some(action.duplicate());
                    *self.best_child = Some(child);
                }

                //handle pruning
                if value >= beta {
                    break;
                }
                alpha = max(alpha, value);
            }
        } else {
            value = f64::INFINITY;

            let mut best_value: Option<f64> = None;
            for m in potential_moves {
                let mut action = m.duplicate();

                if action.is_illegal(&mut test_board) {
                    continue;
                }

                //create child
                let mut child_board = self.board.duplicate();
                action.perform_on(&mut child_board);
                let mut child = Self::new(child_board, self.player);

                //recursion
                let recursion_result = child.alphabeta(depth - 1, alpha, beta, true);
                value = min(value, recursion_result);

                //find best child
                if self.best_move.is_none() {
                    self.best_move = Some(action);
                    best_value = Some(child.evaluate());

                    //put the child in a box
                    *self.best_child = Some(child);
                    continue;
                }

                //otherwise, check for better
                child.value = child.evaluate();
                let unwrapped_best_value = best_value.unwrap();
                if child.value >= unwrapped_best_value {
                    best_value = Some(child.value);
                    self.best_move = Some(action.duplicate());
                    *self.best_child = Some(child);
                }

                if value <= alpha {
                    break;
                }
                beta = min(beta, value);
            }
        }

        self.value = value;
        value
    }

    fn evaluate(&self) -> f64 {
        self.board.evaluation() * self.player.value() as f64
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn take_queen() {
        let mut b = Board::new();
        b.turn = board::Color::Black;

        //set up test board
        let rook_loc = Coordinate::new(0, 0);
        let black_king_loc = Coordinate::new(0, 7);
        let white_king_loc = Coordinate::new(7, 7);
        let queen_loc = Coordinate::new(1, 0);
        let rook = Piece {
            color: board::Color::Black,
            ptype: board::PieceType::Rook,
            location: rook_loc,
            has_moved: true,
        };
        let black_king = Piece {
            color: board::Color::Black,
            ptype: board::PieceType::King,
            location: black_king_loc,
            has_moved: true,
        };
        let white_king = Piece {
            color: board::Color::White,
            ptype: board::PieceType::King,
            location: white_king_loc,
            has_moved: true,
        };
        let queen = Piece {
            color: board::Color::White,
            ptype: board::PieceType::Queen,
            location: queen_loc,
            has_moved: true,
        };
        b.put_piece_on(&rook_loc, rook);
        b.put_piece_on(&black_king_loc, black_king);
        b.put_piece_on(&white_king_loc, white_king);
        b.put_piece_on(&queen_loc, queen);

        //take queen?
        let mut tree = SearchTree::new(&b, 3);
        let maybe_move = tree.best_move();
        let action = maybe_move.unwrap();
        assert_eq!(action.to_coordinate(), Some(Coordinate::new(1, 0)));
    }
}

impl fmt::Debug for SearchNode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SearchNode with value {}, board evaluation {}, and position:\n{}", self.value, self.board.evaluation(), self.board.draw())
    }
}
