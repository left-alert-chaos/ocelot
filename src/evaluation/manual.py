"""Module containing all custom evaluation checks.

# Functions

## all_manual(game: physical.board.Board, turn: physical.board.PieceColor) -> float
Runs all manual checks on board and returns net result.

## check_knights_on_rim(game: physical.board.Board, score: float)
Checks for knights on the rim of the board and changes score.

## pieces_from_color(game: board.Board, ptype: board.PieceType, color: board.PieceColor) -> list[board.Piece]
Finds pieces with given color and type.

## white_total_piece_value(game: physical.board.Board) -> int
How many piece points white has.

## black_total_piece_value(game: physical.board.Board) -> int
How many piece points black has.

## non_predictive(game: physical.board.Board, turn: physical.board.PieceColor) -> float
Returns a float of current tempo/who's winning (positive for white, negative for black).
turn represents whose move it is.
"""

from physical import board


def all_manual(game: board.Board, turn: board.PieceColor) -> float:
    score = 0.0
    
    check_knights_on_rim(game, score)
    
    return score


def non_predictive(game: board.Board, turn: board.PieceColor) -> float:
    #start with material
    score = white_total_piece_value(game) - black_total_piece_value(game)
    score += all_manual(game, turn)
    return float(score)


def white_total_piece_value(game: board.Board) -> int:
    value = 0
    for piece in game.white_pieces():
        value += piece.ptype.value
    return value


def black_total_piece_value(game: board.Board) -> int:
    value = 0
    for piece in game.black_pieces():
        value += piece.ptype.value
    return value
    

def pieces_from_color(game: board.Board, ptype: board.PieceType, color: board.PieceColor) -> list[board.Piece]:
    pieces = []
    for piece in game.pieces:
        if piece.ptype == ptype and piece.color == color:
            pieces.append(piece)
    return pieces


def check_knights_on_rim(game: board.Board, score: float):
    for square in game["a"] + game["h"]:
        if square.piece != None and square.piece.ptype == board.PieceType.KNIGHT:
            score += 0.5 if square.piece.color == board.PieceColor.BLACK else -0.5

