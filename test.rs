use std::ops::Index;
#[derive(Debug, Clone)]
pub struct Board<'a> {
    pub(crate) squares: HashMap<char, [Square<'a>; 8]>,
    pub(crate) pieces: Vec<Piece<'a>>,
}

impl<'a> Index<&'a char> for Board<'a> {
    type Output = &'a [Square<'a>; 8];
    fn index(&self, s: &char) -> &&'a [Square<'a>; 8] {
        if self.squares.contains_key(s) {
            if let Some(column) = self.squares.get(s) {
                &&*column
            } else {
                panic!("Board.squares.get_mut(s) returned None!")
            }
        } else {
            panic!("Board.sqaures has no key {s}");
        }
    }
}
