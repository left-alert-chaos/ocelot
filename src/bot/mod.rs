//!# bot
//!This module holds code to wrap search and eval and move gen into an accessible API. It also holds
//!the UCI implementation.

mod uci;
pub use uci::ToUCI;
use crate::physical::*;
use crate::evaluation::SearchTree;
use crate::bot::uci::FromUCI;
use std::io;

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
    player: board::Color,
    depth: i32,
    options: EngineOptions,
}

impl Ocelot {
    pub fn new(board: &Board, player: board::Color, depth: i32) -> Self {
        Self {
            board: board.duplicate(),
            player,
            depth,
            options: EngineOptions::new(),
        }
    }

    ///This tries to get the best move with a SearchTree. If it can't find one, it returns the first
    ///legal one.
    ///Remember that this doesn't apply the move, just generates it.
    pub fn safe_best_move(&mut self) -> Box<dyn Action> {
        let search_result = self.best_move();
        if let Ok(action) = search_result {
            return action;
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
            "isready" => {} //TODO: wait for background search to finish, if I ever implement
                               //background search
            "setoption" => {} //TODO: Implement option setting
            "position" => {
                self.position(command_tail);
            }
            _ => {
                eprintln!("Ocelot::interpret_uci(): Command {command} isn't implemented.");
            }
        }

        true
    }

    fn position(&mut self, repr: &str) {
        let result = Board::parse(repr.to_string());
        if let Ok(board) = result {
            self.board = board;
        }
    }

    fn best_move(&self) -> Result<Box<dyn Action>, ()> {
        let mut tree = SearchTree::new(&self.board, self.depth);
        tree.best_move()
    }
}
