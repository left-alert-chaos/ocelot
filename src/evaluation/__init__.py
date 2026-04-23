"""A module holding all board evaluation.

# Modules

## manual
Holds many manual checks, like knight on the rim, etc

## search
Holds logic for multi-move prediction and evaluation.

# Functions

## evaluate_move(move: physical.movement.Move, game: physical.board.Board) -> float
Evaluates the current board.
Plays the move on an identical test board and returns the change in evaluation, assuming it's a legal move.
Infers player color from piece being moved.
"""

#path wrangling
import os
import sys
sys.path.append(os.path.dirname(__file__))
from manual import white_total_piece_value, black_total_piece_value, non_predictive
import search
from physical import movement, board
import copy


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

    #flip score for black
    return after_score - before_score


class EvaluationException(Exception):
    """Exception to raise when evaluation fails."""
    def __init__(self, message: str):
        super().__init__(message)

