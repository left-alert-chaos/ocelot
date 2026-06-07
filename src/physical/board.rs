//!Holds peices, piece information, board, etc.
//!
//!As a reminder, all row numbers are 0-indexed; what GothamChess would call a8 is a7 here, and a1
//!is a0.

use std::fmt;

const LETTERS: &str = "abcdefgh";

///# PieceType
///Represents a piece type.
///
///# Public Methods
///
///## value(&self) -> i32
///Returns the value of a piece (1, 3, 5, 9, 50)
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
    pub fn value(&self) -> i32 {
        match self {
            PieceType::Pawn => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 50,
        }
    }

    fn letter(&self) -> char {
        match self {
            PieceType::Pawn => 'p',
            PieceType::Knight => 'n',
            PieceType::Bishop => 'b',
            PieceType::Rook => 'r',
            PieceType::Queen => 'q',
            PieceType::King => 'k',
        }
    }

}

///# Color
///Represents a color. Used for square colors and piece colors.
///
///# Public Methods
///
///## value(&self) -> i32
///returns 1 for White and -1 for Black
#[derive(Debug, Eq, PartialEq, Clone, Default, Copy)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    pub fn value(&self) -> i32 {
        match self {
            Color::White => 1,
            Color::Black => -1,
        }
    }

    fn letter(&self) -> char {
        match self {
            Color::White => 'w',
            Color::Black => 'b',
        }
    }
}

///# Coordinate
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
        write!(f, "Coordinate {col}{row}")
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

    pub fn new(col: char, row: usize) -> Self {
        Coordinate { row, col }
    }
}

///# Square
///One square on the board.
#[derive(Debug, Clone, Default)]
pub struct Square {
    location: Coordinate,
    pub(crate) color: Color,
    pub(crate) piece: Option<Piece>,
}

impl Square {
    //dependency of Board::draw()
    fn draw(&self) -> String {
        //find ansi codes
        let bg = if self.color == Color::Black {"\x1b[40m"} else {"\x1b[47m"};
        let fg = if self.color == Color::Black {"\x1b[0;37m"} else {"\x1b[30m"};
        let piece = if let Some(p) = self.piece {
            p.draw()
        } else {
            String::from("  ")
        };
        
        format!("{bg}{fg}{piece}\x1b[0m")
    }
}

///# Piece
///A piece on a square.
#[derive(Debug, Clone, Copy)]
pub struct Piece {
    pub(crate) color: Color,
    pub(crate) ptype: PieceType,
}

impl Piece {
    //dependency of Square::draw()
    fn draw(&self) -> String {
        format!("{}{}", self.color.letter(), self.ptype.letter())
    }
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
///## new() -> Board
///Creates a new board.
///
///## populate_starting_pos(self) -> Board
///Consumes self and returns a new board with pieces in starting pos.
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
///
///## draw(&self) -> String
///Draws a crude ascii board to represent the current position.
///The fmt::Display implementation uses draw().
#[derive(Debug, Clone)]
pub struct Board {
    squares: [[Square; 8]; 8],
    locations: Vec<Coordinate>, //stores locations of pieces
    pub(crate) turn: Color,
}

impl Board {
    pub fn new() -> Self {
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
            squares: grid,
            locations,
            turn: Color::White,
        }
    }

    pub fn populate_starting_pos(mut self) -> Self {
        //go through each column and put corresponding piece
        self.populate_column('a', PieceType::Rook);
        self.populate_column('h', PieceType::Rook);
        self.populate_column('b', PieceType::Knight);
        self.populate_column('g', PieceType::Knight);
        self.populate_column('c', PieceType::Bishop);
        self.populate_column('f', PieceType::Bishop);
        self.populate_column('d', PieceType::Queen);
        self.populate_column('e', PieceType::King);

        self 
    }

    //this is private because it's just a dependency of populate_starting_pos()
    fn populate_column(&mut self, col: char, ptype: PieceType) {
        //place non-pawns
        self.put_piece_on(&Coordinate::new(col, 0), Piece {
            color: Color::White,
            ptype,
        });
        self.put_piece_on(&Coordinate::new(col, 7), Piece {
            color: Color::Black,
            ptype,
        });

        //pawns
        self.put_piece_on(&Coordinate::new(col, 1), Piece {
            color: Color::White,
            ptype: PieceType::Pawn,
        });
        self.put_piece_on(&Coordinate::new(col, 6), Piece {
            color: Color::Black,
            ptype: PieceType::Pawn,
        });
    }
    
    pub fn square(&self, coord: &Coordinate) -> &Square {
        &self.squares[col_num(coord.col)][coord.row]
    }

    pub fn mut_square(&mut self, coord: &Coordinate) -> &mut Square {
        &mut self.squares[col_num(coord.col)][coord.row]
    }

    pub fn remove_on(&mut self, coord: &Coordinate) {
        self.squares[col_num(coord.col)][coord.row].piece = None;
    }

    pub fn put_piece_on(&mut self, coord: &Coordinate, piece: Piece) {
        let s = self.mut_square(coord);
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
                eprintln!("Board::pieces(): Location {loc} is in Board.locations, but there is no piece there!");
            }
        }

        pieces
    }

    pub fn draw(&self) -> String {
        let mut output = String::from("    a    b    c    d    e    f    g    h");

        //don't use for loop because decreasing ranges are hard
        let mut row: i32 = 7;
        while row >= 0 {
            //row number
            output.push_str(format!("\n\n{}", row + 1).as_str());

            for col in LETTERS.chars() {
                output.push_str(" | ");
                //get square, draw, convert to str
                output.push_str(self.square(&Coordinate::new(col, row as usize)).draw().as_str());
            }

            row -= 1;
        }
        
        output
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
