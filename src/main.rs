mod physical;
mod evaluation;
use physical::{Action, Castle, CastleSide, Color, Move};
use evaluation::SearchTree;

fn main() {
    let mut b = physical::Board::new();
    b.populate_starting_pos();

    let tree = SearchTree::new(&b, 5);
    let maybe_best_move = tree.best_move();
    let best_move = match maybe_best_move {
        Ok(m) => format!("{}", m),
        Err(_) => String::from("No best move!"),
    };
    println!("Default position:\n{}\n\nBest move: {}", b.draw(), best_move);
}
