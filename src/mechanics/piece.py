from enum import Enum


class PieceColor(Enum):
    """Enum representing piece colors. BLACK is 0 and WHITE is 1. It probably doesn't even need a docstring. Bye!"""
    BLACK = 0
    WHITE = 1


class Piece:
    """A piece on the board."""
