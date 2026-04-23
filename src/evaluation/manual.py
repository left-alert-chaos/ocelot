"""# manual
Module containing all custom evaluation checks.
All heuristics are written here.

# Functions

## all_manual(game: physical.board.Board, turn: physical.board.PieceColor) -> float
Runs all manual checks on board and returns net result.

## check_knights_on_rim(game: physical.board.Board, score: float)
Checks for knights on the rim of the board and changes score.

## pawns_in_center(game: phyisical.board.Board, score: float)
Checks for pawns in center 4 squares and awards 1 point each.

## pieces_from_color(game: board.Board, ptype: board.PieceType, color: board.PieceColor) -> list[board.Piece]
Finds pieces with given color and type.

## white_total_piece_value(game: physical.board.Board) -> int
How many piece points white has.

## black_total_piece_value(game: physical.board.Board) -> int
How many piece points black has.

## non_predictive(game: physical.board.Board, turn: physical.board.PieceColor) -> float
Returns a float of current tempo/who's winning (positive for white, negative for black).
turn represents whose move it is.

# Classes

## StalemateException(Exception)
Exception raised when the player whose turn it is has no legal moves.
"""

from physical import board, movement


def all_manual(game: board.Board, turn: board.PieceColor) -> float:
    score = 0.0
    
    check_knights_on_rim(game, score)
    pawns_in_center(game, score)
    
    return score


def non_predictive(game: board.Board, turn: board.PieceColor=board.PieceColor.WHITE) -> float:
    #end of game?
    white_moves = movement.white_legal_moves(game)
    black_moves = movement.black_legal_moves(game)
    opponent = board.PieceColor.BLACK if turn == board.PieceColor.WHITE else board.PieceColor.WHITE
    player_moves, opponent_moves = (white_moves, black_moves) if turn == board.PieceColor.WHITE else (black_moves, white_moves)
    if len(player_moves) == 0:
        if movement.is_check(turn, game):
            return float("-inf")
        else:
            raise StalemateException(turn)
    elif len(opponent_moves) == 0:
        if movement.is_check(opponent, game):
            return float("inf")
        else:
            raise StalemateException(opponent)

    #start with material
    score = white_total_piece_value(game) - black_total_piece_value(game)
    score += all_manual(game, turn)

    if turn == board.PieceColor.BLACK:
        score *= -1

    return score


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


def pawns_in_center(game: board.Board, score: float):
    #private function to look at a square for pawns
    def check_square(square: board.Square, score: float):
        if square.piece == None:
            return
        if square.piece.ptype == board.PieceType.PAWN:
            score += 1 if square.piece.color == board.PieceColor.BLACK else -1

    check_square(game["d"][3], score)
    check_square(game["d"][4], score)
    check_square(game["e"][3], score)
    check_square(game["e"][4], score)


class StalemateException(Exception):
    """# StalemateException
    Error raised when a person has no legal moves.

    # Methods

    ## __init__(self, color: physical.board.PieceColor)
    Initializes the exception."""
    
    def __init__(self, color: board.PieceColor):
        super().__init__(f"Player {color.name} has no legal moves.")

