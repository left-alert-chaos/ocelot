"""A module holding all board evaluation.

# Modules

## manual
Holds many manual checks, like knight on the rim, etc

# Functions

## non_predictive(game: physical.board.Board, turn: physical.board.PieceColor) -> float
Returns a float of current tempo/who's winning (positive for white, negative for black).
turn represents whose move it is.

## evaluate_move(move: physical.movement.Move, game: physical.board.Board) -> float
Evaluates the current board.
Plays the move on an identical test board and returns the change in evaluation, assuming it's a legal move.
Infers player color from piece being moved.

## white_total_piece_value(game: physical.board.Board) -> int
How many piece points white has.

## black_total_piece_value(game: physical.board.Board) -> int
How many piece points black has.
"""

#path wrangling
import os
import sys
sys.path.append(os.path.dirname(__file__))
import manual
from physical import movement, board
import copy


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


def evaluate_move(move: movement.Move | movement.Castle, game: board.Board) -> float:
    #determine player color
    if isinstance(move, movement.Move):
        moving_piece = game[move.from_col][move.from_row].piece
        if moving_piece == None:
            raise EvaluationException("Tried to evaluate move from empty square.")
        player_color = moving_piece.color
    else:
        #the move is a castle
        player_color = move.color
    opponent_color = board.PieceColor.WHITE if player_color == board.PieceColor.BLACK else board.PieceColor.BLACK
    
    before_score = non_predictive(game, player_color)
    
    test_game = copy.deepcopy(game)
    move.perform_on(test_game)
    after_score = non_predictive(test_game, opponent_color)

    net = after_score - before_score
    return net if player_color == board.PieceColor.WHITE else -1 * net


class EvaluationException(Exception):
    """Exception to raise when evaluation fails."""
    def __init__(self, message: str):
        super().__init__(message)

