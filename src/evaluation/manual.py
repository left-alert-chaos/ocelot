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

## castling(game: physical.board.Board, score: float)
Awards points for castling

## freedom(game: physical.board.Board, score: float)
An experimental heuristic that awards points for being able to move to more squares.

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
    castling(game, score)
    #freedom(game, score)
    
    return score


def non_predictive(game: board.Board, turn: board.PieceColor=board.PieceColor.WHITE) -> float:
    white_moves = movement.white_legal_moves(game)
    black_moves = movement.black_legal_moves(game)
    opponent = board.PieceColor.BLACK if turn == board.PieceColor.WHITE else board.PieceColor.WHITE
    player_moves, opponent_moves = (white_moves, black_moves) if turn == board.PieceColor.WHITE else (black_moves, white_moves)

    #check for end of game and hard-code scores
    if len(player_moves) == 0:
        if movement.is_check(turn, game):
            return float("-inf")
        else:
            return 0.0
    elif len(opponent_moves) == 0:
        if movement.is_check(opponent, game):
            return float("inf")
        else:
            return 0.0

    #start with material
    score = white_total_piece_value(game) - black_total_piece_value(game)

    if turn == board.PieceColor.BLACK:
        score *= -1

    score += all_manual(game, turn)

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
            score += square.piece.color.value


def pawns_in_center(game: board.Board, score: float):
    #private function to look at a square for pawns
    def check_square(square: board.Square, score: float):
        if square.piece == None:
            return
        if square.piece.ptype == board.PieceType.PAWN:
            score += square.piece.color.value

    check_square(game["d"][3], score)
    check_square(game["d"][4], score)
    check_square(game["e"][3], score)
    check_square(game["e"][4], score)


def castling(game: board.Board, score: float):
    if game.white_castled: score += 2
    elif game.black_castled: score -= 2


def freedom(game: board.Board, score: float):
    score += (
            #reward being able to move to more places by subtracting lengths of legal move lists
            len(movement.white_legal_moves(game)) - len(movement.black_legal_moves(game))
    ) * 0.1


class StalemateException(Exception):
    """# StalemateException
    Error raised when a person has no legal moves.

    # Methods

    ## __init__(self, color: physical.board.PieceColor)
    Initializes the exception."""
    
    def __init__(self, color: board.PieceColor):
        super().__init__(f"Player {color.name} has no legal moves.")

