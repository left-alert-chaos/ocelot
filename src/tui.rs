//!# tui
//!A simple implementation of a TUI for Ocelot.

use crate::physical::*;
use crate::bot::Ocelot;
use crate::bot::safe_parse_action;
use std::io;

//make board renderable in ascii
impl Board {
    fn draw_ascii(&self, x: u32, mut y: u32) -> String {
        let plain  = self.draw();
        let mut output = String::new();

        for line in plain.split("\n") {
            output.push_str(format!("\x1b[{y};{x}H{line}").as_str());
            y += 1;
        }

        output
    }
}

struct TUIState {
    board: Board,
    engine: Ocelot,
    error: String,
}

impl TUIState {
    ///Render the UI
    fn render(&mut self) {
        //erase screen and buffer. disable line wrap
        print!("\x1b[H\x1b[2J\x1b[3J\x1b[=7l");

        //padding at the top
        println!();

        //Logo
        println!("    \x1b[1;33m\x1b[3mOcelot\x1b[0m\n    Depth: {}", self.engine.depth);

        //info
        println!("\nOcelot is a simple(ish) chess engine.\nPlay by typing a long algebraic notation move. Examples:\n\n- e2e4 To move your pawn to e4\n- e1g1 To castle kingside\n- h7h8q to promote a pawn on h7 to queen.\n\nThe pieces are represented by 2-letter strings:\nthe color and the type.\nFor example,\nthe white rook at the bottom-left corner is represented as \"wr\".\n\nTo quit, type \"quit\".\n");

        //error
        println!("\x1b[1;31m{}\x1b[0m", self.error);

        //draw board and return cursor to input location
        println!("{}\x1b[1A", self.board.draw_ascii(70, 4));

        self.error = String::new();
    }
}

pub fn mainloop(depth: i32) {
    //switch to alternative screen buffer
    print!("\x1b[?1049h");
    let board = Default::default();

    let mut state = TUIState {
        engine: Ocelot::new(&board, depth),
        board,
        error: String::new(),
    };


    //actual main loop
    loop {
        //render
        state.render();

        //get user input
        let mut input = String::new();
        let _ = io::stdin().read_line(&mut input);
        input = input.trim().to_string();

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
            if user_action.is_illegal(&mut state.board) {
                state.error = String::from("That's an illegal move!");
                continue;
            }

            user_action.perform_on(&mut state.board);
            state.engine.perform_on_self(user_action);

            state.error = String::from("Engine is thinking...");
            state.render();

            let mut engine_move = state.engine.safe_best_move();
            engine_move.perform_on(&mut state.board);
            state.engine.perform_on_self(engine_move);
        } else {
            //bad move
            state.error = String::from("Couldn't parse that move!");
        }
    }

    //switch to main buffer before exit
    print!("\x1b[?1049l");
}

