mod physical;

fn main() {
    let mut b = physical::Board::new();
    b = b.populate_starting_pos();
    println!("{}", b.draw());
}
