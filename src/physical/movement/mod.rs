//!# movement
//!This module holds logic to generate and check moves for legality. Most major logic is in impl's
//!for structs defined elsewhere.
//!This file is mostly tests.

pub mod types;
pub mod board;
pub mod generation;

pub use types::{Action, Move, Castle, MoveInfo, CastleSide};

#[cfg(test)]
mod tests {
    use crate::physical::{board, Board, Coordinate, Piece};

    #[test]
    fn no_rook_moves() {
        //no rook moves at the start
        let b: Board = Default::default();

        let c = Coordinate::new(0, 7); //black rook
        let square = b.square(&c);
        let piece = square.piece.unwrap();
        assert_eq!(piece.potential_moves(&b).len(), 0);
    }

    #[test]
    fn only_castling() {
        let b: Board = Default::default();

        let c = Coordinate::new(4, 0); //white king
        let square = b.square(&c);
        let piece = square.piece.unwrap();
        assert_eq!(piece.potential_moves(&b).len(), 2);
    }

    #[test]
    fn two_knight_moves() {
        let b: Board = Default::default();

        let c = Coordinate::new(1, 0);
        let square = b.square(&c);
        let piece = square.piece.unwrap();
        assert_eq!(piece.potential_moves(&b).len(), 2);
    }

    #[test]
    fn thirty_two_locations() {
        let b: Board = Default::default();
        assert_eq!(b.locations.len(), 32);
    }

    #[test]
    //like with king moves, castling is added to be filtered for legality later. that adds 2
    //elements.
    fn twenty_two_opening_moves() {
        let b: Board = Default::default();

        assert_eq!(b.white_potential_moves().len(), 22);
        assert_eq!(b.black_potential_moves().len(), 22);
    }

    #[test]
    fn twenty_legal_opening_moves() {
        let mut b: Board = Default::default();

        let mut white_moves = b.white_potential_moves();
        white_moves.retain_mut(|x| !x.is_illegal(&mut b));
        let mut black_moves = b.black_potential_moves();
        black_moves.retain_mut(|x| !x.is_illegal(&mut b));

        assert_eq!(white_moves.len(), 20);
        assert_eq!(black_moves.len(), 20);
    }


    #[test]
    fn eight_knight_moves() {
        let mut b = Board::new();
        let c = Coordinate::new(4, 4);
        let p = Piece {
            color: board::Color::White,
            ptype: board::PieceType::Knight,
            location: c,
            has_moved: false,
        };
        b.put_piece_on(&c, p);

        assert_eq!(p.potential_moves(&b).len(), 8);
    }

    #[test]
    fn pawn_captures_and_promotes() {
        let mut b = Board::new();
        let pawn_loc = Coordinate::new(0, 6);
        let rook_loc = Coordinate::new(1, 7);
        let pawn = Piece {
            color: board::Color::White,
            ptype: board::PieceType::Pawn(0),
            location: pawn_loc,
            has_moved: true,
        };
        let rook = Piece {
            color: board::Color::Black,
            ptype: board::PieceType::Rook,
            location: rook_loc,
            has_moved: true,
        };
        b.put_piece_on(&pawn_loc, pawn);
        b.put_piece_on(&rook_loc, rook);
        assert_eq!(pawn.potential_moves(&b).len(), 8);
    }

    #[test]
    fn pawn_captures() {
        let mut b = Board::new();
        let pawn_loc = Coordinate::new(0, 6);
        let rook_loc = Coordinate::new(1, 5);
        let pawn = Piece {
            color: board::Color::Black,
            ptype: board::PieceType::Pawn(0),
            location: pawn_loc,
            has_moved: false,
        };
        let rook = Piece {
            color: board::Color::White,
            ptype: board::PieceType::Rook,
            location: rook_loc,
            has_moved: true,
        };
        b.put_piece_on(&pawn_loc, pawn);
        b.put_piece_on(&rook_loc, rook);
        assert_eq!(pawn.potential_moves(&b).len(), 3);
    }

    #[test]
    fn en_passant() {
        let mut b = Board::new();
        let jumping_pawn_loc = Coordinate::new(1, 4);
        let capture_pawn_loc = Coordinate::new(0, 4);
        let jumping_pawn = Piece {
            color: board::Color::Black,
            ptype: board::PieceType::Pawn(1),
            location: jumping_pawn_loc,
            has_moved: true,
        };
        let capture_pawn = Piece {
            color: board::Color::White,
            ptype: board::PieceType::Pawn(0),
            location: capture_pawn_loc,
            has_moved: true,
        };
        b.put_piece_on(&jumping_pawn_loc, jumping_pawn);
        b.put_piece_on(&capture_pawn_loc, capture_pawn);
        assert_eq!(capture_pawn.potential_moves(&b).len(), 2);
    }

    #[test]
    fn is_check() {
        let mut b = Board::new();
        let king_loc = Coordinate::new(4, 0);
        let rook_loc = Coordinate::new(0, 0);
        let king = Piece {
            color: board::Color::White,
            ptype: board::PieceType::King,
            location: king_loc,
            has_moved: true,
        };
        let rook = Piece {
            color: board::Color::Black,
            ptype: board::PieceType::Rook,
            location: rook_loc,
            has_moved: true,
        };
        b.put_piece_on(&king_loc, king);
        b.put_piece_on(&rook_loc, rook);
        b.update();
        println!(
            "Black threatened squares: {:?}",
            b.move_info.black_threatened_squares
        );
        assert_eq!(b.is_check(board::Color::White), true);
    }
}
