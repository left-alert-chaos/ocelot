//!Holds peices, piece information, board, etc.
//!
//!As a reminder, all row numbers are 0-indexed; what GothamChess would call a8 is a7 here, and a1
//!is a0.

#[allow(unused_imports)]
use std::{
    collections::HashMap,
    fmt::{Display, Debug},
    ops::{Index, IndexMut},
};

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
    #[default] White,
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

impl Color {
    fn coordinate_color(row: &u8, col: &char) -> Option<Color> {
        let col_num = LETTERS.find(*col)?;

        //if the row num plus the col num is even, return black. else white
        if (col_num as u8 + *row) % 2 == 0 {return Some(Color::Black)} else {return Some(Color::White)};
    }
}

///One square on the board.
#[derive(Debug, Clone, Default)]
pub struct Square<'a> {
    pub(crate) row: u8,
    pub(crate) col: char,
    pub(crate) color: Color,
    pub(crate) piece: Option<Piece<'a>>,
}

///A piece on a square.
#[derive(Debug, Clone, Copy)]
pub struct Piece<'a> {
    pub(crate) color: Color,
    pub(crate) ptype: PieceType,
    pub(crate) location: &'a Square<'a>,
}

#[derive(Debug, Clone)]
pub struct Board<'a> {
    pub(crate) squares: [[Square<'a>; 8]; 8],
}

impl Default for Board<'_> {
    fn default() -> Self {
        let mut arr = Vec::new();

        for col in LETTERS.chars() {
            //build vec of squares and convert to array
            let mut col_vec = Vec::new();
            for row in 0..8 {
                //get square color
                let color = Color::coordinate_color(&row, &col).unwrap_or_else(|| Color::White);

                //create square
                col_vec.push(Square {
                    row,
                    col,
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
        };

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
        }
    }
}

impl<'a> Board<'a> {
    fn square(&self, col: char, row: usize) -> &Square<'_> {
        &self.squares[col_num(col)][row]
    }

    fn put_piece_on(&'a mut self, col: char, row: usize, piece: Piece<'a>) {
        self.squares[col_num(col)][row].piece = Some(piece);
    }
}

pub fn col_num(name: char) -> usize {
    let res = LETTERS.find(name);
    if let Some(num) = res {
        return num
    } else {
        panic!("Column {name} doesn't exist!");
    }
}
