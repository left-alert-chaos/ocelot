"""A module containing clean APIs that abstract away all other messiness. Just a bot that plays chess.

# Classes

## Sophisticate
The bot.

## GameException(Exception)
An exception for use when no legal moves could be found or some other reason prevents a move from being found or played.

## WinException(GameException)
An exception raised if the bot wins.

## LossException(GameException)
An exception raised if the bot loses.

## StalemateException(GameException)
An exception raised if no legal moves could be found but there isn't check."""

import os
import sys
dirname = os.path.dirname(__file__)
if dirname not in sys.path:
    sys.path.append(dirname)
from physical import movement, board
import evaluation
import random


class Sophisticate:
    """A chess bot that can play one side of the game.

    # Methods

    ## __init__(self, game: physical.board.Board, color: physical.board.PieceColor)
    Initializes the bot to find the best moves for the given side.

    ## best_move(self) -> physical.movement.Move:
    (hopefully) finds the best legal move for the bot's color. It does NOT play the move, only returning it.
    """

    def __init__(self, game: board.Board, color: board.PieceColor):
        self.game = game
        self.color = color

    def best_move(self) -> movement.Move:
        #score moves
        pieces = self.game.white_pieces() if self.color == board.PieceColor.WHITE else self.game.black_pieces()
        moves = []
        for piece in pieces:
            moves += movement.potential_moves(piece, self.game)
        moves = [move for move in moves if not move.is_illegal(self.game)]

        #if this isn't suffled, it always plays the same game given tieing moves.
        random.shuffle(moves)
        if len(moves) == 0:
            if movement.is_check(self.color, self.game):
                raise LossException("I lost by checkmate!")
            else:
                raise StalemateException("I don't have any legal moves, but I'm not in check.")

        best_move = moves[0]
        for move in moves:
            move.value = evaluation.evaluate_move(move, self.game)
            if move.value >= best_move.value:
                best_move = move
        return best_move


class GameException(Exception):
    """An exception for use when no legal moves could be found or some other reason prevents a move from being found or played."""
    def __init__(self, message: str):
        super().__init__(message)


class WinException(GameException):
    """An exception raised when the bot wins."""
    def __init__(self, message: str):
        super().__init__(message)


class LossException(GameException):
    """An exception raised when the bot loses."""
    def __init__(self, message: str):
        super().__init__(message)


class StalemateException(GameException):
    """An exception raised when no legal moves could be found."""
    def __init__(self, message: str):
        super().__init__(message)

