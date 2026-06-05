//!Holds peices, piece information, board, etc.
//!
//!As a reminder, all row numbers are 0-indexed; what GothamChess would call a8 is a7 here, and a1
//!is a0.

use std::fmt;

const LETTERS: &str = "abcdefgh";

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    fn value(&self) -> u32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 50,
        }
    }
}

#[derive(Debug, Eq, PartialEq, Clone, Default, Copy)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    fn value(&self) -> i32 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }
}

///Represents the location of a square or piece. All fields are public.
#[derive(Debug, Clone, Copy, Default)]
pub struct Coordinate {
    pub(crate) row: usize,
    pub(crate) col: char,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let col = self.col;
        let row = self.row;
        write!(f, "Coordinate at {col}{row}")
    }
}

impl Coordinate {
    fn color(&self) -> Option<Color> {
        let col_num = LETTERS.find(self.col)?;

        //if the row num plus the col num is even, return black. Else, white.
        if (col_num + self.row).is_multiple_of(2) {
            Some(Color::Black)
        } else {
            Some(Color::White)
        }
    }
}

impl Coordinate {
    pub fn new(col: char, row: usize) -> Self {
        Coordinate { row, col }
    }
}

///One square on the board.
#[derive(Debug, Clone, Default)]
pub struct Square {
    location: Coordinate,
    pub(crate) color: Color,
    pub(crate) piece: Option<Piece>,
}

///A piece on a square.
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub(crate) color: Color,
    pub(crate) ptype: PieceType,
}

///# Board
///The board the game is played on.
///
///# Public fields
///
///## turn: Color
///The player whose turn it is.
///
///# Public Methods
///
///## square(&self, col: char, row: usize) -> &Square<'_>
///Returns an immutable reference to the square at the specified locationl.
///
///## mut_square(&mut self, col: char, row: usize) -> &mut Square<'_>
///Like `square()` but returns mutable reference.
///
///## put_piece_on(&'a mut self, col: char, row: usize, piece: Piece<'a>)
///Consumes `piece` and puts it on the specified square.
///
///## pieces(&mut self) -> Vec<&Piece<'a>>
///Iterates through all squares on board to find pieces.
///Returns Vec of mutable references to the pieces.
///Use sparingly, because it runs in O(n) time.
///
///## white_pieces(&mut self) -> Vec<&Piece<'a>>
///Similar to `pieces()`, but returns only white pieces.
///
///## black_pieces(&mut self) -> Vec<&Piece<'a>>
///Similar to `pieces()`, but returns only black pieces.
#[derive(Debug, Clone)]
pub struct Board {
    squares: [[Square; 8]; 8],
    iter_col: usize, //use usize because otherwise a lot of conversion between col num and char
    //would be necessary to check for out of bounds when incrementing.
    iter_row: usize,
    locations: Vec<Coordinate>, //stores locations of pieces
    pub(crate) turn: Color,
}

impl Default for Board {
    fn default() -> Self {
        let mut arr = Vec::new();
        let mut locations = Vec::new();

        for col in LETTERS.chars() {
            //build vec of squares and convert to array
            let mut col_vec = Vec::new();
            for row in 0..8 {
                let coord = Coordinate { row, col };
                locations.push(coord);
                //get square color
                let color = coord.color().unwrap_or(Color::White);

                //create square
                col_vec.push(Square {
                    location: coord,
                    color,
                    piece: None,
                })
            }

            //now that vec has been built, attempt to convert to array and add to squares HashMap
            let maybe_array = col_vec.try_into();
            if let Ok(col_array) = maybe_array {
                arr.push(col_array)
            } else {
                panic!("Couldn't create squares array for column {col}");
            }
        }

        //convert vec to array
        let grid;
        let maybe_grid = arr.try_into();
        if let Ok(grid_array) = maybe_grid {
            grid = grid_array;
        } else {
            panic!("Couldn't convert grid vector to array.");
        }

        Board {
            iter_col: 0,
            iter_row: 0,
            squares: grid,
            locations,
            turn: Color::White,
        }
    }
}

impl Board {
    pub fn square(&self, coord: &Coordinate) -> &Square {
        &self.squares[col_num(coord.col)][coord.row]
    }

    pub fn mut_square(&mut self, col: char, row: usize) -> &mut Square {
        &mut self.squares[col_num(col)][row]
    }

    pub fn remove_on(&mut self, coord: Coordinate) {
        self.squares[col_num(coord.col)][coord.row].piece = None;
    }

    pub fn put_piece_on(&mut self, col: char, row: usize, piece: Piece) {
        let s = self.mut_square(col, row);
        s.piece = Some(piece);
    }

    ///Keep in mind that `pieces()` returns *copies* of pieces on the board, not the pieces
    ///themselves.
    pub fn pieces(&mut self) -> Vec<Piece> {
        let mut pieces = Vec::new();

        for loc in self.locations.iter() {
            if let Some(piece) = self.square(loc).piece {
                pieces.push(piece)
            } else {
                eprintln!("Location {loc} is in Board.locations, but there is no piece there!");
            }
        }

        pieces
    }
}

pub fn col_num(name: char) -> usize {
    let res = LETTERS.find(name);
    if let Some(num) = res {
        num
    } else {
        panic!("Column {name} doesn't exist!");
    }
}
