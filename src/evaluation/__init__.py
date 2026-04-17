"""A module holding all board evaluation.

# Modules

## manual
Holds many manual checks, like knight on the rim, etc

# Functions

## non_predictive(game: physical.board.Board, turn: physical.board.PieceColor) -> float
Returns a float of current tempo/who's winning (positive for white, negative for black).
turn represents whose move it is.

## white_total_piece_value(game: physical.board.Board) -> int
How many piece points white has.

## black_total_piece_value(game: physical.board.Board) -> int
How many piece points black has.
"""

#path wrangling
import os
import sys
dirname = os.path.dirname(__file__)
if dirname not in sys.path:
    sys.path.append(dirname)
import manual
from physical import movement, board


def non_predictive(game: board.Board, turn: board.PieceColor) -> float:
    #start with material
    score = white_total_piece_value(game) - black_total_piece_value(game)
    score += manual.all_manual(game, turn)
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

