mod physical;
mod evaluation;
mod bot;
mod tui;
use bot::Ocelot;
use physical::*;
use std::env;

//parse args
fn main() {
    let mut depth = 5;

    let args: Vec<_> = env::args().collect();

    //look for depth arguments
    for arg in args.iter() {
        match arg.parse::<i32>() {
            Ok(num) => {
                depth = num;
                break;
            },
            Err(_) => continue,
        }
    }

    if args.contains(&String::from("--demo")) {
        demo(depth);
    } else if args.contains(&String::from("--tui")) {
        tui::mainloop(depth);
    } else {
        uci(depth);
    }
}

//start a UCI bot
fn uci(depth: i32) {
    let b: Board = Default::default();
    let mut engine = Ocelot::new(&b, depth);
    engine.uci_loop();
}

fn demo(depth: i32) {
    let b: Board = Default::default();

    println!("Depth: {depth}");
    let mut white_engine = Ocelot::new(&b, depth);
    let mut black_engine = Ocelot::new(&b, depth);

    println!("This demo will play a game between equally-strengthed White and Black players until you kill the program. It gets chaotic and a little illegal!");
    
    //play indefinitely
    loop {
        println!("Round {}", white_engine.board.round);
        let white_move = white_engine.safe_best_move();
        println!("White plays {white_move}");
        white_engine.perform_on_self(white_move.duplicate());
        black_engine.perform_on_self(white_move);
        println!("White in check: {}", b.is_check(board::Color::White));

        let black_move = black_engine.safe_best_move();
        println!("Black plays {black_move}");
        white_engine.perform_on_self(black_move.duplicate());
        black_engine.perform_on_self(black_move);
        println!("Black in check: {}", b.is_check(board::Color::Black));
    }
}
