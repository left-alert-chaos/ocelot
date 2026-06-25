mod physical;
mod evaluation;
mod bot;
use bot::Ocelot;
use physical::*;

fn main() {
    let mut b = physical::Board::new();
    b.populate_starting_pos();

    let depth = 5;
    println!("Depth: {depth}");
    let mut white_engine = Ocelot::new(&b, board::Color::White, depth);
    let mut black_engine = Ocelot::new(&b, board::Color::Black, depth);

    println!("This demo will play a game between equally-strengthed White and Black players until you kill the program. It gets chaotic and a little illegal!");
    
    //play indefinitely
    loop {
        println!("Round {}", white_engine.board.round);
        let white_move = white_engine.safe_best_move();
        println!("White plays {white_move}");
        white_engine.perform_on_self(white_move.duplicate());
        black_engine.perform_on_self(white_move);

        let black_move = black_engine.safe_best_move();
        println!("Black plays {black_move}");
        white_engine.perform_on_self(black_move.duplicate());
        black_engine.perform_on_self(black_move);
    }
}
