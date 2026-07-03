//!# tui
//!A simple implementation of a TUI for Ocelot.

use crate::bot::Ocelot;
use crate::bot::safe_parse_action;
use crate::physical::*;
use std::io;

//make board renderable in ascii
impl Board {
    fn draw_ascii(&self, x: u32, mut y: u32) -> String {
        let plain = self.draw();
        let mut output = String::new();

        for line in plain.split("\n") {
            output.push_str(format!("\x1b[{y};{x}H{line}").as_str());
            y += 1;
        }

        output
    }
}

///Stores the persistent state of the interface, including latest move and whether to exit
struct TUIState {
    board: Board,
    engine: Ocelot,
    error: String,
    to_exit: bool,
    engine_last_move: Option<Box<dyn Action>>,
}

impl TUIState {
    ///Render the UI
    fn render(&mut self) {
        //erase screen and buffer. disable line wrap
        print!("\x1b[H\x1b[2J\x1b[3J\x1b[=7l");

        //padding at the top
        println!();

        //Logo
        println!(
            "    \x1b[1;33m\x1b[3mOcelot\x1b[0m\n    Depth: {}",
            self.engine.depth
        );

        //info
        println!(
            "\nOcelot is a simple(ish) chess engine.\nPlay by typing a long algebraic notation move. Examples:\n\n- e2e4 To move your pawn to e4\n- e1g1 To castle kingside\n- h7h8q to promote a pawn on h7 to queen.\n\nThe pieces are represented by 2-letter strings:\nthe color and the type.\nFor example,\nthe white rook at the bottom-left corner is represented as \"wr\".\n\nTo quit, type \"quit\".\n"
        );

        //error
        println!("\x1b[1;31m{}\x1b[0m", self.error);

        let engine_move = match &self.engine_last_move {
            Some(m) => m.generate(),
            None => String::from("none"),
        };

        //draw board and engine move and return cursor to input location
        println!(
            "{}\x1b[22;70HEngine move: {engine_move}\x1b[3A",
            self.board.draw_ascii(70, 4)
        );

        self.error = String::new();
    }

    fn white_potential_moves_contains(&self, action: &Box<dyn Action>) -> bool {
        for potential in &self.board.move_info.white_potential_moves {
            if action.is_equal_to(&potential) {
                return true;
            }
        }

        false
    }
}

pub fn mainloop(depth: i32) {
    //switch to alternative screen buffer
    print!("\x1b[?1049h");
    let board = Default::default();

    let mut state = TUIState {
        engine: Ocelot::new(&board, depth, 8.0),
        board,
        error: String::new(),
        to_exit: false,
        engine_last_move: None,
    };

    //actual main loop
    loop {
        //check if player has moves
        let mut player_moves = state.board.white_potential_moves();
        player_moves.retain_mut(|x| !x.is_illegal(&mut state.board));
        if player_moves.len() == 0 {
            state.error = if state.board.is_check(board::Color::White) {
                String::from("You were checkmated!")
            } else {
                String::from("Stalemate!")
            };

            state.to_exit = true;
        }

        //render
        state.render();

        //get user input
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        input = input.trim().to_string();

        if state.to_exit {
            break;
        }

        //filter empty lines
        if input == String::from("") {
            state.error = String::from("Nothing was typed!");
            continue;
        }

        if input.to_lowercase() == String::from("quit") {
            break;
        }

        //perform move, get engine response
        if let Ok(mut user_action) = safe_parse_action(input, &mut state.board) {
            if user_action.is_illegal(&mut state.board) || !state.white_potential_moves_contains(&user_action) {
                state.error = String::from("That's an illegal move!");
                continue;
            }

            user_action.perform_on(&mut state.board);
            state.engine.perform_on_self(user_action);

            //check if engine has moves
            let mut engine_moves = state.board.black_potential_moves();
            engine_moves.retain_mut(|x| !x.is_illegal(&mut state.board));
            if engine_moves.len() == 0 {
                state.error = if state.board.is_check(board::Color::Black) {
                    String::from("You checkmated Ocelot!")
                } else {
                    String::from("Stalemate!")
                };

                state.to_exit = true;
                continue;
            }

            state.error = String::from("Engine is thinking...");
            state.render();

            let mut engine_move = state.engine.safe_best_move();
            engine_move.perform_on(&mut state.board);
            state.engine.perform_on_self(engine_move.duplicate());
            state.engine_last_move = Some(engine_move);
        } else {
            //bad move
            state.error = String::from("Couldn't parse that move!");
        }
    }

    //switch to main buffer before exit
    print!("\x1b[?1049l");
}

