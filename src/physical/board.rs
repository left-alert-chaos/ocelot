//!Holds peices, piece information, board, etc.
//!
//!As a reminder, all row numbers are 0-indexed; what GothamChess would call a8 is a7 here, and a1
//!is a0.

use std::{
    collections::HashMap,
    fmt::{Display, Debug}
};

const LETTERS: &str = "abcdefgh";

#[derive(Debug, Eq, PartialEq, Clone)]
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

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum Color {
    #[default] White,
    Black,
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
    pub(crate) piece: Option<&'a Piece<'a>>,
}

///A piece on a square.
#[derive(Debug, Clone)]
pub struct Piece<'a> {
    pub(crate) color: Color,
    pub(crate) ptype: PieceType,
    pub(crate) location: &'a Square<'a>,
}

#[derive(Debug, Clone)]
pub struct Board<'a> {
    pub(crate) squares: HashMap<char, [Square<'a>; 8]>,
    pub(crate) pieces: Vec<Piece<'a>>,
}

impl Default for Board<'_> {
    fn default() -> Self {
        //CREATE SQUARES
        let mut squares = HashMap::new();
        
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
            if let Ok(arr) = maybe_array {
                squares.insert(col, arr);
            } else {
                panic!("Couldn't create squares array for column {col}");
            }
        };

        Board {
            squares,
            pieces: vec![]
        }
    }
}

//lots of lifetime generics to imply that everything works
/*
impl<'a> Index<&'a char> for Board<'a> {
    type Output = &'a [Square<'a>; 8];
    fn index(&self, col_name: &'a char) -> &&'a [Square<'a>; 8] {
        if self.squares.contains_key(col_name) {
            if let Some(column) = self.squares.get(col_name) {
                &&column
            } else {
                panic!("Board.squares.get({col_name}) returned None!")
            }
        } else {
            panic!("Board.sqaures has no key {col_name}");
        }
    }
}
*/

/*
impl<'a> Index<&'a char> for Board<'a> {
    type Output = &'a Option<&'a [Square<'a>; 8]>;
    fn index(&self, col_name: &'a char) -> &&'a Option<&'a [Square<'a>; 8]> {
        if self.squares.contains_key(col_name) {
            &&(*self).squares.get(col_name)
        } else {
            panic!("Board.squares.get({col_name}) returned None!");
        }
    }
}
*/

//impl IndexMut<&'_ str> for Board<'_> {
//    type Output = 
//}

#[allow(dead_code)]
impl Board<'_> {
    fn populate_column(&mut self, col_name: &char, ptype: PieceType) {
        //let mut col = self.
    }
    /*
    fn mutable_column<'a>(&mut self, col_name: &char) -> &mut'a [Square; 8] {
        if self.squares.contains_key(col_name) {
            if let Some(column) = self.squares.get_mut(col_name) {
                &mut column
            } else {
                panic!("Board.squares.get({col_name}) returned None!");
            }
        } else {
            panic!("Board.squares has no key {col_name}");
        }
    }
    */

    /*
    fn new_mutable_column<'a>(&mut self, col_name: &char) -> Option<&'a mut [Square; 8]> {
        self.squares.get_mut(col_name)
    }
    */

    ///If the coordinates are valid, a &Square<'_> is returned.
    ///Invalid coordinates panic.
    fn get(&self, col_name: &char, row: u8) -> &Square<'_> {
        let lookup = self.squares.get(col_name);
        if let Some(col) = lookup {
            &col[row as usize]
        } else {
            panic!("Board<'_>::get({col_name}, {row}): No such column as {col_name}!");
        }
    }
    
    ///If the coordinates are valid, the Square's piece field is set.
    ///Actually physically replaces the preexisting Square to get around lifetime issues.
    ///Invalid coordinates panic.
    fn set<'a>(&'a mut self, col_name: &char, row: u8, piece: &Piece<'a>) {
        let lookup = self.squares.get_mut(col_name);
        let Some(col) = lookup else {
            panic!("Board<'_>::set({col_name}, {row}, {:?}): No such column as {col_name}", piece);
        };
        col[row as usize].piece = Some(piece);
    }
}
