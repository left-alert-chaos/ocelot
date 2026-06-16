//!# board
//!Holds peices, piece information, board, etc.
//!
//!As a reminder, all row numbers are 0-indexed; what GothamChess would call a8 is a7 here, and a1
//!is a0.

use std::fmt;
use std::mem::{self, MaybeUninit};

pub const LETTERS: &str = "abcdefgh";

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
#[derive(Eq, PartialEq, Clone, Default, Copy)]
pub enum Color {
    #[default]
    White,
    Black,
}

impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let word = match self {
            Self::White => "White",
            Self::Black => "Black",
        };
        write!(f, "{word}")
    }
}

impl fmt::Debug for Color {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{self}")
    }
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
///
///# Public Methods
///
///## is_valid(&self) -> bool
///Determines whether coordinate fits into an 8 by 8 board
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Coordinate {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coordinate col = {} row = {}", self.col, self.row)
    }
}

impl Coordinate {
    pub fn is_valid(&self) -> bool {
        self.row < 8 && self.col < 8
    }

    fn color(&self) -> Option<Color> {
        //if the row num plus the col num is even, return black. Else, white.
        if (self.col + self.row).is_multiple_of(2) {
            Some(Color::Black)
        } else {
            Some(Color::White)
        }
    }

    pub fn new(col: usize, row: usize) -> Self {
        Coordinate { row, col }
    }
}

///# Square
///One square on the board.
///
///# Public Methods
///
///## value(&self) -> i32
///The value of the piece on the square. If there is no piece, value is 0.
#[derive(Debug, Copy, Clone, Default, Eq, PartialEq)]
pub struct Square {
    pub(crate) location: Coordinate,
    pub(crate) color: Color,
    pub(crate) piece: Option<Piece>,
}

impl Square {
    pub fn value(&self) -> i32 {
        if let Some(piece) = self.piece {
            piece.ptype.value()
        } else {
            0
        }
    }

    //dependency of Board::draw()
    fn draw(&self) -> String {
        //find ansi codes
        let bg = if self.color == Color::Black {
            "\x1b[40m"
        } else {
            "\x1b[47m"
        };
        let fg = if self.color == Color::Black {
            "\x1b[0;37m"
        } else {
            "\x1b[30m"
        };
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
///
///# Public Methods
///
///## potential_moves(&self, game: &board::Board) -> Vec<Action>
///Gets potential moves that don't collide, but doesn't check for legality
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct Piece {
    pub(crate) color: Color,
    pub(crate) ptype: PieceType,
    pub(crate) location: Coordinate,
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
///## populate_starting_pos(&mut self)
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
///## remove_piece_on(&mut self, coord: &Coordinate)
///Deletes the piece at the coordinate and removes coordinate from piece locations list.
///
///## move_from(&mut self, from: &Coordinate, to: &Coordinate)
///Moves the piece on from to the square at to.
///Locations are managed automatically.
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
#[derive(Debug, Clone, Eq)]
pub struct Board {
    squares: [[Square; 8]; 8],
    pub(crate) locations: Vec<Coordinate>, //stores locations of pieces
    pub(crate) turn: Color,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.draw())
    }
}

impl PartialEq for Board {
    //VERY INEFFICENT
    fn eq(&self, other: &Self) -> bool {
        if self.turn != other.turn {
            return false;
        } else if self.squares != other.squares {
            return false;
        } else {
            //search locations
            for loc in self.locations.clone() {
                if !other.locations.contains(&loc) {
                    return false;
                }
            }
        }

        true
    }
}

impl Board {
    pub fn new() -> Self {
        let mut uninit_arr: [MaybeUninit<[Square; 8]>; 8] = [MaybeUninit::uninit(); 8];
        let mut locations = Vec::new();

        for col in 0..8 {
            //build vec of squares and convert to array
            let mut uninit_col_array: [MaybeUninit<Square>; 8] = [MaybeUninit::uninit(); 8];

            for row in 0..8 {
                let coord = Coordinate::new(col, row);
                locations.push(coord);
                //get square color
                let color = coord.color().unwrap_or(Color::White);

                //create square
                uninit_col_array[row].write(Square {
                    location: coord,
                    color,
                    piece: None,
                });
            }

            unsafe {
                let col_array = mem::transmute::<_, [Square; 8]>(uninit_col_array);
                uninit_arr[col].write(col_array);
            }
        }

        //convert uninitialized cols into array
        let grid;
        unsafe {
            grid = mem::transmute::<_, [[Square; 8]; 8]>(uninit_arr);
        }

        Board {
            squares: grid,
            locations,
            turn: Color::White,
        }
    }

    pub fn populate_starting_pos(&mut self) {
        //go through each column and put corresponding piece
        self.populate_column(0, PieceType::Rook);
        self.populate_column(7, PieceType::Rook);
        self.populate_column(1, PieceType::Knight);
        self.populate_column(6, PieceType::Knight);
        self.populate_column(2, PieceType::Bishop);
        self.populate_column(5, PieceType::Bishop);
        self.populate_column(3, PieceType::Queen);
        self.populate_column(4, PieceType::King);
    }

    //this is private because it's just a dependency of populate_starting_pos()
    fn populate_column(&mut self, col: usize, ptype: PieceType) {
        //place non-pawns
        let c = Coordinate::new(col, 0);
        self.put_piece_on(
            &c,
            Piece {
                color: Color::White,
                ptype,
                location: c,
            },
        );
        let c = Coordinate::new(col, 7);
        self.put_piece_on(
            &c,
            Piece {
                color: Color::Black,
                ptype,
                location: c,
            },
        );

        //pawns
        let c = Coordinate::new(col, 1);
        self.put_piece_on(
            &c,
            Piece {
                color: Color::White,
                ptype: PieceType::Pawn,
                location: c,
            },
        );
        let c = Coordinate::new(col, 6);
        self.put_piece_on(
            &c,
            Piece {
                color: Color::Black,
                ptype: PieceType::Pawn,
                location: c,
            },
        );
    }

    pub fn square(&self, coord: &Coordinate) -> &Square {
        &self.squares[coord.col][coord.row]
    }

    pub fn mut_square(&mut self, coord: &Coordinate) -> &mut Square {
        &mut self.squares[coord.col][coord.row]
    }

    pub fn remove_on(&mut self, coord: &Coordinate) {
        self.squares[coord.col][coord.row].piece = None;
    }

    pub fn put_piece_on(&mut self, coord: &Coordinate, mut piece: Piece) {
        let s = self.mut_square(coord);
        piece.location = *coord;
        s.piece = Some(piece);
        self.locations.push(*coord);
    }

    pub fn remove_piece_on(&mut self, coord: &Coordinate) {
        //delete piece and remove location from locations
        self.mut_square(coord).piece = None;
        self.locations.retain(|loc| loc != coord);
    }

    pub fn move_from(&mut self, from: &Coordinate, to: &Coordinate) {
        let from_square = self.mut_square(from);
        let moving_piece = from_square.piece.expect(
            format!("Moving from {from} to {to} isn't possible because there is no piece to move.")
                .as_str(),
        );

        self.remove_on(from);
        self.put_piece_on(to, moving_piece);
    }

    ///Keep in mind that `pieces()` returns *copies* of pieces on the board, not the pieces
    ///themselves.
    pub fn pieces(&mut self) -> Vec<Piece> {
        let mut pieces = Vec::new();

        for loc in self.locations.iter() {
            if let Some(piece) = self.square(loc).piece {
                pieces.push(piece)
            } else {
                eprintln!(
                    "Board::pieces(): Location {loc} is in Board.locations, but there is no piece there!"
                );
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
                output.push_str(
                    self.square(&Coordinate::new(col_num(col), row as usize))
                        .draw()
                        .as_str(),
                );
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
