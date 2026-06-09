mod physical;
use physical::{Move, Action};

fn main() {
    let mut b = physical::Board::new();
    b.populate_starting_pos();
    
    //move king to center of board because why not
    let m = Move::new(physical::Coordinate::new('e', 0), physical::Coordinate::new('e', 3), &b, None, false);
    m.perform_on(&mut b);
    println!("Behold: A king in the center of the board!\n{}", b.draw());
}
