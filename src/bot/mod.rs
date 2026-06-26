//!# bot
//!This module holds code to wrap search and eval and move gen into an accessible API. It also holds
//!the UCI implementation.

mod uci;
pub use uci::ToUCI;
use crate::physical::*;
use crate::evaluation::SearchTree;
use std::io;
use std::fs;

//TODO: add config options
struct EngineOptions {

}

impl EngineOptions {
    fn new() -> Self {
        Self {}
    }
}

///# Ocelot
///The whole reason we're here. This struct keeps track of game state and uses SearchTrees to
///generate the best move for the player the bot is assigned.
pub struct Ocelot {
    pub(crate) board: Board,
    depth: i32,
    options: EngineOptions,
    ponder: Option<Box<dyn Action>>,
}

impl Ocelot {
    pub fn new(board: &Board, depth: i32) -> Self {
        Self {
            board: board.duplicate(),
            depth,
            options: EngineOptions::new(),
            ponder: None,
        }
    }

    ///This tries to get the best move with a SearchTree. If it can't find one, it returns the first
    ///legal one.
    ///Remember that this doesn't apply the move, just generates it.
    pub fn safe_best_move(&mut self) -> Box<dyn Action> {
        let mut tree = SearchTree::new(&self.board, self.depth);
        tree.safe_best_move()
    }

    pub fn perform_on_self(&mut self, mut action: Box<dyn Action>) {
        action.perform_on(&mut self.board);
    }

    ///Main engine loop. Get input, find move, etc
    pub fn uci_loop(&mut self) {
        loop {
            let mut input = String::new();
            let _ = io::stdin().read_line(&mut input);

            //interpret command and determine whether to end loop
            if !self.interpret_uci(input) {
                break
            }
        }
    }

    //returns whether to continue loop
    fn interpret_uci(&mut self, input: String) -> bool {
        let mut words = input.split_whitespace();

        //get command
        let Some(command) = words.nth(0) else {
            return true;
        };

        let command_tail = input.split_at(command.len() + 1).1;

        //main decision tree
        match command {
            "quit" => return false,
            "uci" => {
                println!("uciok");
            }
            "ucinewgame" => {},
            "isready" => {println!("readyok");} //TODO: wait for background search to finish, if I ever implement
                               //background search
            "setoption" => {} //TODO: Implement option setting
            "position" => {
                self.position(command_tail);
            }
            "go" => {
                self.go(command_tail);
            }
            "ponderhit" => {
                //perform pondered move
                if let Some(pondered) = &mut self.ponder {
                    pondered.perform_on(&mut self.board);
                }
            }
            "stop" => {} //TODO: If background eval is ever implemented, use this to kill it
            "d" => {
                println!("{}", self.board.draw());
            }
            _ => {
                eprintln!("Ocelot::interpret_uci(): Command {command} isn't implemented.");
            }
        }

        true
    }

    //get the move and play it
    fn go(&mut self, _command: &str) {
        let mut tree = SearchTree::new(&self.board, self.depth);
        let mut best_move = tree.safe_best_move();
        let maybe_best_position = tree.root.best_child;

        //get pondered move
        let ponder = if let Some(best_pos) = *maybe_best_position {
            if let Some(ponder) = best_pos.best_move {
                ponder.generate()
            } else {
                String::from("0000")
            }
        } else {
            String::from("0000")
        };

        best_move.perform_on(&mut self.board);
        println!("bestmove {} ponder {ponder}", best_move.generate());
    }

    fn position(&mut self, repr: &str) {
        let result = Board::parse(repr.to_string());
        if let Ok(board) = result {
            self.board = board;
        }
    }
}
