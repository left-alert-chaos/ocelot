//!# board
//!Holds peices, piece information, board, etc.
//!
//!As a reminder, all row numbers are 0-indexed; what GothamChess would call a8 is a7 here, and a1
//!is a0.

use crate::physical::movement::MoveInfo;
use std::fmt;
use std::mem::{self, MaybeUninit};
use std::collections::HashSet;

pub const LETTERS: &str = "abcdefgh";

///# PieceType
///Represents a piece type.
///
///# Public Methods
///
///## value(&self) -> i32
///Returns the value of a piece (1, 3, 5, 9, 50)
///
///## from(letter: char) -> Result<Self, ()>
///Creates Self from piece letter
///
///## letter(&self) -> char
///Gets a character representing the piece type.
#[derive(Debug, Eq, PartialEq, Clone, Copy)]
pub enum PieceType {
    Pawn(i32), //store turn pawn can be en passant'd on.
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    pub fn value(&self) -> i32 {
        match self {
            PieceType::Pawn(_) => 1,
            PieceType::Knight => 3,
            PieceType::Bishop => 3,
            PieceType::Rook => 5,
            PieceType::Queen => 9,
            PieceType::King => 50,
        }
    }

    pub fn from(letter: char) -> Result<Self, ()> {
        match letter.to_ascii_lowercase() {
            'p' => Ok(Self::Pawn(0)),
            'n' => Ok(Self::Knight),
            'b' => Ok(Self::Bishop),
            'r' => Ok(Self::Rook),
            'q' => Ok(Self::Queen),
            'k' => Ok(Self::King),
            _ => Err(()),
        }
    }

    pub fn letter(&self) -> char {
        match self {
            PieceType::Pawn(_) => 'p',
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
///
///## opposite(&self) -> Self
///If self is white, return black. If self is black, return white.
///
///## home_rank(&self) -> usize
///Returns the 0 or 7
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
    
    pub fn home_rank(&self) -> usize {
        match self {
            Color::White => 0,
            Color::Black => 7,
        }
    }

    pub fn opposite(&self) -> Self {
        match self {
            Color::White => Color::Black,
            Color::Black => Color::White,
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
///
///## with_offset(&self, rise: i32, run: i32) -> Coordinate
///Creates a new coordinate a certain number of rows and columns away from self.
///
///## from(repr: String) -> Coordinate 
///Returns a coordinate from a 2-character string of <col><row>
///This is not 0-indexed, so row is decremented by one.
#[derive(Clone, Copy, Default, PartialEq, Eq, Hash)]
pub struct Coordinate {
    pub(crate) row: usize,
    pub(crate) col: usize,
}

impl fmt::Debug for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Coordinate col = {} row = {}", self.col, self.row)
    }
}

impl fmt::Display for Coordinate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", LETTERS.chars().nth(self.col).unwrap_or('a'), self.row + 1)
    }
}

impl Coordinate {
    pub fn is_valid(&self) -> bool {
        self.row < 8 && self.col < 8
    }

    pub fn with_offset(&self, rise: i32, run: i32) -> Result<Coordinate, ()> {
        let mut new = *self;

        if rise < 0 {
            let offset = -rise as usize;

            //can't leave board
            if offset > new.row {
                return Err(());
            }

            new.row -= offset; //convert to positive number and subtract
        } else {
            new.row += rise as usize;
        }
        if run < 0 {
            let offset = -run as usize;

            //can't leave board
            if offset > new.col {
                return Err(());
            }
            new.col -= offset; //convert to positive number and subtract
        } else {
            new.col += run as usize;
        }

        if new.is_valid() { Ok(new) } else { Err(()) }
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

    pub fn from(repr: &mut String) -> Self {
        let col = col_num(repr.remove(0));
        let row = repr.remove(0).to_string().parse::<usize>();
        
        Self {
            col,
            row: row.unwrap() - 1,
        }
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
    pub(crate) has_moved: bool,
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
///## pieces(&self) -> Vec<Piece>
///Iterates through all squares on board to find pieces.
///Use sparingly, because it runs in O(n) time.
///
///## white_pieces(&self) -> Vec<Piece>
///Uses pieces() but filters for white pieces.
///
///## black_pieces(&self) -> Vec<Piece>
///Uses pieces() but filters for black pieces.
///
///## is_check(&self, player: Color) -> bool
///Is player's king threatened?
///
///## is_checkmate(&self, player: Color) -> bool
///Is check and no legal moves?
///
///## remove_piece_on(&mut self, coord: &Coordinate)
///Deletes the piece at the coordinate and removes coordinate from piece locations list.
///
///## move_from(&mut self, from: &Coordinate, to: &Coordinate)
///Moves the piece on from to the square at to.
///Locations are managed automatically.
///
///## draw(&self) -> String
///Draws a crude ascii board to represent the current position.
///The fmt::Display implementation uses draw().
///
///## update(&mut self)
///Replaces move_info field with up-to-date one.
///
///## duplicate(&self) -> Self
///Creates an identical new Board.
///Runs in O(n) time, so use sparingly.
///
///## round_next_turn(&self) -> i32
///Gets what round it will be next turn
#[derive(Debug)]
pub struct Board {
    squares: [[Square; 8]; 8],
    pub(crate) locations: HashSet<Coordinate>, //stores locations of pieces
    pub(crate) turn: Color,
    pub(crate) round: i32, //the number of the set of 2 moves
    pub(crate) move_info: MoveInfo,
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.draw())
    }
}

impl Default for Board {
    fn default() -> Self {
        let mut new = Self::new();
        new.populate_starting_pos();
        new
    }
}

impl PartialEq for Board {
    //VERY INEFFICENT
    fn eq(&self, other: &Self) -> bool {
        if self.turn != other.turn || self.squares != other.squares {
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

        for (col, col_arr_mem) in uninit_arr.iter_mut().enumerate() {
            //build vec of squares and convert to array
            let mut uninit_col_array: [MaybeUninit<Square>; 8] = [MaybeUninit::uninit(); 8];

            for (row, cell) in uninit_col_array.iter_mut().enumerate() {
                let coord = Coordinate::new(col, row);
                //get square color
                let color = coord.color().unwrap_or(Color::White);

                //create square
                cell.write(Square {
                    location: coord,
                    color,
                    piece: None,
                });
            }

            unsafe {
                let col_array =
                    mem::transmute::<[MaybeUninit<Square>; 8], [Square; 8]>(uninit_col_array);
                col_arr_mem.write(col_array);
            }
        }

        //convert uninitialized cols into array
        let grid;
        unsafe {
            grid = mem::transmute::<[MaybeUninit<[Square; 8]>; 8], [[Square; 8]; 8]>(uninit_arr);
        }

        Board {
            squares: grid,
            locations: HashSet::new(),
            turn: Color::White,
            round: 1, //not 0-indexed; 0 is used to signal "never" or a turn that is impossible
            move_info: MoveInfo::new(),
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
                has_moved: false,
            },
        );
        let c = Coordinate::new(col, 7);
        self.put_piece_on(
            &c,
            Piece {
                color: Color::Black,
                ptype,
                location: c,
                has_moved: false,
            },
        );

        //pawns
        let c = Coordinate::new(col, 1);
        self.put_piece_on(
            &c,
            Piece {
                color: Color::White,
                ptype: PieceType::Pawn(0),
                location: c,
                has_moved: false,
            },
        );
        let c = Coordinate::new(col, 6);
        self.put_piece_on(
            &c,
            Piece {
                color: Color::Black,
                ptype: PieceType::Pawn(0),
                location: c,
                has_moved: false,
            },
        );
    }

    pub fn square(&self, coord: &Coordinate) -> &Square {
        &self.squares[coord.col][coord.row]
    }

    pub fn mut_square(&mut self, coord: &Coordinate) -> &mut Square {
        &mut self.squares[coord.col][coord.row]
    }

    pub fn put_piece_on(&mut self, coord: &Coordinate, mut piece: Piece) {
        let s = self.mut_square(coord);
        piece.location = *coord;
        s.piece = Some(piece);
        self.locations.insert(*coord);
    }

    pub fn remove_on(&mut self, coord: &Coordinate) {
        //delete piece and remove location from locations
        self.mut_square(coord).piece = None;
        self.locations.remove(coord);
    }

    pub fn move_from(&mut self, from: &Coordinate, to: &Coordinate) {
        let from_square = self.mut_square(from);
        let mut moving_piece = from_square.piece.unwrap_or_else(|| {
            panic!("Moving from {from} to {to} isn't possible because there is no piece to move.")
        });
        moving_piece.has_moved = true;

        self.remove_on(from);
        self.put_piece_on(to, moving_piece);
    }

    ///Keep in mind that `pieces()` returns *copies* of pieces on the board, not the pieces
    ///themselves.
    pub fn pieces(&self) -> Vec<Piece> {
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

    pub fn white_pieces(&self) -> Vec<Piece> {
        let mut pieces = self.pieces();
        pieces.retain(|p| p.color == Color::White);
        pieces
    }

    pub fn black_pieces(&self) -> Vec<Piece> {
        let mut pieces = self.pieces();
        pieces.retain(|p| p.color == Color::Black);
        pieces
    }

    pub fn draw(&self) -> String {
        let mut output = String::from("     a    b    c    d    e    f    g    h");

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

    pub fn duplicate(&self) -> Self {
        let mut new = Self::new();

        //iterate over pieces and place on new board
        for piece in self.pieces() {
            new.put_piece_on(&piece.location, piece);
        }

        //set other fields
        new.turn = self.turn;
        new.round = self.round;
        new.update();

        new
    }

    pub fn round_next_turn(&self) -> i32 {
        match self.turn {
            Color::White => self.round,
            Color::Black => self.round + 1,
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    fn default_board() -> Board {
        let mut b = Board::new();
        b.populate_starting_pos();
        b
    }

    #[test]
    fn positive_offset() {
        let c = Coordinate::new(5, 5);
        let new = c.with_offset(1, 1);
        assert_eq!(new, Ok(Coordinate::new(6, 6)));
    }

    #[test]
    fn negative_offset() {
        let c = Coordinate::new(5, 5);
        let new = c.with_offset(-5, -5);
        assert_eq!(new, Ok(Coordinate::new(0, 0)));
    }

    #[test]
    fn num_of_pieces() {
        let b = default_board();
        assert_eq!(b.pieces().len(), 32);
        assert_eq!(b.white_pieces().len(), 16);
        assert_eq!(b.black_pieces().len(), 16);
    }

    #[test]
    fn coordinate_from() {
        let expected = Coordinate::new(0, 0);
        let generated = Coordinate::from(&mut String::from("a1"));
        assert_eq!(generated, expected);
    }
}
