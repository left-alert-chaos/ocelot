//! All physical representation handling.
//!
//! Pieces, types, colors, and the board are handled in the `board` module.
//! Movement, castling, and move generation are handled in the `movement` module.

#[allow(dead_code)]
pub mod board;
pub mod movement;

pub use board::{
    Board,
    Coordinate,
};

pub use movement::{
    Move,
    Action,
};

