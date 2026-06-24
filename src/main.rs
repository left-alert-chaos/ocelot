mod physical;
mod evaluation;
mod bot;
use bot::Ocelot;
use physical::*;

fn main() {
    let mut b = physical::Board::new();
    b.populate_starting_pos();

    let depth = 6;
    println!("Depth: {depth}");
    let mut engine = Ocelot::new(&b, board::Color::White, depth);
    let best_move = engine.safe_best_move();
    println!("Best move for white in default position: {}", best_move);
}
