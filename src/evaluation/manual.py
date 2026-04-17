"""Module containing all custom evaluation checks.

# Functions

## all_manual(game: physical.board.Board, turn: physical.board.PieceColor) -> float
Runs all manual checks on board and returns net result.

## hanging_queen(game: physical.board.Board, turn: physical.board.PieceColor) -> int
Checks if current player can immediately take opponent's queen.
Returns 9 or -9 for hanging and 0 for not.

## pieces_from_color(game: board.Board, ptype: board.PieceType, color: board.PieceColor) -> list[board.Piece]
Finds pieces with given color and type.
"""

from physical import board, movement


def all_manual(game: board.Board, turn: board.PieceColor) -> float:
    score = 0.0
    score += hanging_queen(game, turn)
    return score


def hanging_queen(game: board.Board, turn: board.PieceColor) -> int:
    threats = game.squares_white_threatens if turn == board.PieceColor.WHITE else game.squares_black_threatens
    opponent_queens = pieces_from_color(game, board.PieceType.QUEEN, board.PieceColor.BLACK if turn == board.PieceColor.WHITE else board.PieceColor.WHITE)
    for opponent_queen in opponent_queens:
        if opponent_queen in threats:
            return 9 if turn == board.PieceColor.WHITE else -9
    return 0
    

def pieces_from_color(game: board.Board, ptype: board.PieceType, color: board.PieceColor) -> list[board.Piece]:
    pieces = []
    for piece in game.pieces:
        if piece.ptype == ptype and piece.color == color:
            pieces.append(piece)
    return pieces

