mod physical;
mod evaluation;
use physical::{Action, Castle, CastleSide, Color, Move};

fn main() {
    let mut b = physical::Board::new();
    b.populate_starting_pos();

    //move king to center of board because why not
    let mut m = Move::new(
        physical::Coordinate::new(4, 0),
        physical::Coordinate::new(4, 3),
        &b,
        None,
        false,
    );
    m.perform_on(&mut b);
    println!("Behold: A king in the center of the board!\n{}", b.draw());

    m.undo_on(&mut b);
    println!("Behold: A king where it started!\n{}", b.draw());

    let mut c = Castle::new(CastleSide::KingSide, Color::White);
    c.perform_on(&mut b);
    println!("Behold: A board with a castle!\n{}", b.draw());

    c.undo_on(&mut b);
    println!("Behold: A board without a castle!\n{}", b.draw());

    let mut qc = Castle::new(CastleSide::QueenSide, Color::Black);
    qc.perform_on(&mut b);
    println!("Behold: A board with a queenside castle!\n{}", b.draw());

    qc.undo_on(&mut b);
    println!("Behold: A bold without a queenside castle!\n{}", b.draw());
}
