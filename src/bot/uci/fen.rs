//!# fen
//!This module holds code to parse Forsyth-Edwards Notation.
//!It's all in an impl block for Board.

use crate::physical::*;

#[allow(refining_impl_trait)]
impl Board {
    pub fn parse(representation: String) -> Result<Board, ()> {
        if representation.starts_with("startpos") {
            return Ok(Default::default());
        }

        let mut new = Board::new();

        //closure to automate placement
        let mut put_piece_on_board =
            |color: board::Color, ptype: board::PieceType, location: Coordinate| {
                //kings and rooks haven't moved by default. This is overwritten later when it parses the castling
                //legality section.
                let has_moved = match ptype {
                    board::PieceType::Pawn(_) => location.row != color.pawn_home_rank(),
                    board::PieceType::Rook | board::PieceType::King => true,
                    _ => false,
                };

                new.put_piece_on(
                    &location,
                    Piece {
                        color,
                        ptype,
                        location,
                        has_moved,
                    },
                );
            };

        //closure to automate has_moved setting to make castling legal
        //takes board as an argument to avoid borrowing issues
        let make_legal = |castle: Castle, new: &mut Board| {
            println!("Making {castle} legal!");
            //get the location of the castler's king
            let king_location = Coordinate::new(4, castle.player.home_rank());

            //make the king to have not moved
            //get the king, set has_moved, and replace
            #[allow(unused_assignments)]
            if let Some(piece) = new.mut_square(&king_location).piece {
                let mut king = piece;
                println!("Setting has_moved for piece at {king_location}");
                king.has_moved = false;
                new.put_piece_on(&king_location, king);
            }

            //get rook position and make it to have not moved
            let rook_col = castle.side.rook_start_col();
            let rook_loc = Coordinate::new(rook_col, castle.player.home_rank());

            //get the rook, set has_moved, and replace
            if let Some(piece) = new.mut_square(&rook_loc).piece {
                let mut rook = piece;
                println!("Setting has_moved for piece at {rook_loc}");
                rook.has_moved = false;
                new.put_piece_on(&rook_loc, rook);
            }
        };

        //get terms in representation
        let mut coord = Coordinate::new(0, 7);
        let mut repr_index = 0;

        //build board and place pieces
        for character in representation.chars() {
            match character {
                //black pieces
                'p' => put_piece_on_board(board::Color::Black, board::PieceType::Pawn(0), coord),
                'r' => put_piece_on_board(board::Color::Black, board::PieceType::Rook, coord),
                'n' => put_piece_on_board(board::Color::Black, board::PieceType::Knight, coord),
                'b' => put_piece_on_board(board::Color::Black, board::PieceType::Bishop, coord),
                'q' => put_piece_on_board(board::Color::Black, board::PieceType::Queen, coord),
                'k' => put_piece_on_board(board::Color::Black, board::PieceType::King, coord),

                //white pieces
                'P' => put_piece_on_board(board::Color::White, board::PieceType::Pawn(0), coord),
                'R' => put_piece_on_board(board::Color::White, board::PieceType::Rook, coord),
                'N' => put_piece_on_board(board::Color::White, board::PieceType::Knight, coord),
                'B' => put_piece_on_board(board::Color::White, board::PieceType::Bishop, coord),
                'Q' => put_piece_on_board(board::Color::White, board::PieceType::Queen, coord),
                'K' => put_piece_on_board(board::Color::White, board::PieceType::King, coord),

                //flow control
                '/' => {
                    coord.row -= 1;
                    coord.col = 0;
                    repr_index += 1;
                    continue;
                }
                ' ' => break,
                _ => {
                    //parse jumps
                    if character.is_ascii_digit() {
                        let digit = character.to_digit(10);
                        if let Some(amount) = digit {
                            coord.col += amount as usize;
                        }
                    }
                    repr_index += 1;
                    continue;
                }
            }

            coord.col += 1;
            repr_index += 1;
        }

        //get whose turn it is
        repr_index += 1;
        let turn_char = match representation.chars().nth(repr_index) {
            Some(turn) => turn,
            None => return Err(()),
        };
        let turn = match board::Color::from(turn_char.to_string()) {
            Ok(color) => color,
            Err(_) => return Err(()),
        };

        repr_index += 1;

        //determine castling ability and set if kings and rooks have moved
        for _ in 0..4 {
            repr_index += 1;

            //get character. If there is none, invalid FEN and can't generate
            let castle_char = match representation.chars().nth(repr_index) {
                Some(c) => c,
                None => return Err(()),
            };

            match castle_char {
                'K' => make_legal(
                    Castle::new(CastleSide::KingSide, board::Color::White),
                    &mut new,
                ),
                'Q' => make_legal(
                    Castle::new(CastleSide::QueenSide, board::Color::White),
                    &mut new,
                ),
                'k' => make_legal(
                    Castle::new(CastleSide::KingSide, board::Color::Black),
                    &mut new,
                ),
                'q' => make_legal(
                    Castle::new(CastleSide::QueenSide, board::Color::Black),
                    &mut new,
                ),
                ' ' => break,
                '-' => {
                    //ensure it ends on a space
                    repr_index += 1;
                    break;
                }
                _ => continue,
            }
        }

        repr_index += 1;

        //get en passant representation
        let mut en_passant_string = String::new();
        for _ in 0..2 {
            let character = match representation.chars().nth(repr_index) {
                Some(c) => c,
                None => return Err(()),
            };

            match character {
                ' ' => break,
                _ => en_passant_string.push(character),
            }
        }

        if !en_passant_string.contains('-') && en_passant_string.len() == 2 {
            //Find pawn
            let en_passant_location = Coordinate::from(&mut en_passant_string);
            let pawn_row = match en_passant_location.row {
                2 => 3,
                5 => 4,
                _ => return Err(()),
            };
            let pawn_location = Coordinate::new(en_passant_location.col, pawn_row);

            //set whether it can be en passant'ed
            if let Some(piece) = new.square(&pawn_location).piece {
                let mut pawn = piece;
                pawn.ptype = board::PieceType::Pawn(1);
                new.put_piece_on(&pawn_location, pawn);
            }
        }

        new.turn = turn;

        Ok(new)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::physical::Square;

    fn diff(generated: [[Square; 8]; 8], expected: [[Square; 8]; 8]) {
        let mut differences = 0;
        for col in 0..8 {
            for row in 0..8 {
                let generated_square = generated[col][row];
                let expected_square = expected[col][row];

                if generated_square != expected_square {
                    differences += 1;
                    println!("\n\nDifference at col {col} and row {row}.");
                    if let Some(piece) = generated_square.piece {
                        println!("Generated piece: {piece:?}");
                    } else {
                        println!("Didn't generate a piece.");
                    }

                    if let Some(piece) = expected_square.piece {
                        println!("Expected piece:  {piece:?}");
                    } else {
                        println!("Didn't expect a piece.");
                    }
                }
            }
        }
        println!("{differences} differences found.");
    }

    #[test]
    fn parse_default_pos_fen() {
        let expected: Board = Default::default();
        let generated = Board::parse(String::from(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        ))
        .unwrap();
        println!(
            "drawing generated board (should be default position):\n{}",
            generated.draw()
        );
        diff(generated.squares, expected.squares);
        assert_eq!(generated, expected);
    }

    #[test]
    fn parse_e4_pos_fen() {
        let mut expected: Board = Default::default();

        //move pawn to e4
        let pawn_start = Coordinate::new(4, 1);
        let pawn_end = Coordinate::new(4, 3);
        expected.move_from(&pawn_start, &pawn_end);
        expected.turn = board::Color::Black;

        //generate board
        let generated = Board::parse(String::from(
            "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1",
        ))
        .unwrap();
        assert_eq!(generated, expected);
    }
}
