"""Module that holds anything for moves.

# Classes

## Action
An empty class that represents any board action.

## Move
Represents a single move on a board and inherits from Action.

## Castle
Represents castling (either side) and inherits from Action.

## CastleSide
Enum representing side to castle to. Values are QUEEN and KING.

## MoveException(Exception)
Error for when moves fail.

# Functions

## potential_moves(piece: board.Piece, game: board.Board) -> list[Move]
Finds potential moves for pieces. Does not find only legal moves; doesn't skip pinned pieces and doesn't skip moves that don't end check.

## <piece>_moves(piece: board.Piece, game: board.Board) -> list[Move]
Dependency of potential_moves(). Same shtick, but only for the given piece.

## king_square_value(square: board.Square, king_color: board.PieceColor) -> None | int
Checks if a square has a collision, or is capturable. If None is returned, the square cannot be moved to.
If an integer is returned, it is the value of capturing on that square.

## king_square(square: board.Square, king_color: board.PieceColor, allowed_squares: list[tuple[board.Square, int]])
Runs king_square_value and checks for None. If an integer is returned, the square is appended to allowed_squares."""

import board
import copy
from enum import Enum


class Action:
    """Empty class representing any kind of action on the board. All move and action types inherit from Action."""
    pass


class CastleSide(Enum):
    """Enum representing sides to castle to.

    # Values

    QUEEN = 1
    KING = 0"""

    QUEEN = 1
    KING = 0


class Castle(Action):
    """Class representing a castle action. Inherits from Action.

    # Methods

    ## __init__(self, side: CastleSide, color: board.PieceType)
    Initializes. The color argument represents the player that is castling.

    ## perform_on(self, game: board.Board)
    Castles on the given board. Does *NOT* check for legality, but does check for whether king and rooks have moved."""
    def __init__(self, side: CastleSide, color: board.PieceColor):
        super().__init__()
        self.side = side
        self.color = color

    def perform_on(self, game: board.Board):
        pieces = game.white_pieces() if self.color == board.PieceColor.WHITE else game.black_pieces()
        king = None
        rooks = []

        # Find all relevant pieces (king, rooks)
        for piece in pieces:
            match piece.ptype:
                case board.PieceType.KING:
                    king = piece
                case board.PieceType.ROOK:
                    if not piece.has_moved:
                        rooks.append(piece)
        
        # Check for previous movements
        if king == None:
            raise MoveException(f"Could not find a {self.color.name} king, so castling is impossible.")
        if king.has_moved:
            raise MoveException(f"{king} has already moved, so castling is illegal.")
        if len(rooks) == 0:
            raise MoveException(f"No {self.color.name} rooks could be found that haven't already moved, so castling is illegal.")
        
        backrank = king.location.row

        if self.side == CastleSide.QUEEN:
            # move king to new pos
            king.location.piece = None
            game["c"][backrank].piece = king
            king.location = game["c"][backrank]

            # move rook to new pos
            rook = game["a"][backrank].piece
            if rook == None:
                raise MoveException(f"Somewhere, somehow, the rook on ({game['a'][backrank]}) became None.")
            rook.location.piece = None
            game["d"][backrank].piece = rook
            rook.location = game["d"][backrank]
        else:
            # move king to new pos
            king.location.piece = None
            game["g"][backrank].piece = king
            king.location = game["g"][backrank]

            # move rook to new pos
            rook = game["h"][backrank].piece
            if rook == None:
                raise MoveException(f"Somewhere, somehow, the rook on ({game['a'][backrank]}) became None.")
            rook.location.piece = None
            game["f"][backrank].piece = rook
            rook.location = game["f"][backrank]

            

    def __str__(self) -> str:
        return f"{self.color.name} castles {self.side.name}side."

    def __repr__(self) -> str:
        return f"{self.color.name} castles {self.side.name}side."


class Move(Action):
    """Simple class to represent a move.

    # Methods

    ## __init__(self, from_col: str, from_row: int, to_col: str, to_row: int)
    Pretty self-explanatory.

    ## perform_on(self, game: board.Board)
    Checks whether a piece is on from square. If there isn't, raises an exception. Else, moves the piece.

    ## is_illegal(self, game: board.Board)
    Checks for illegal move (moving pinned piece, etc) and returns True if illegal is False if legal.
    """

    def __init__(self, from_col: str, from_row: int, to_col: str, to_row: int, value: int=0):
        super().__init__()
        self.from_col = from_col
        self.from_row = from_row
        self.to_col = to_col
        self.to_row = to_row
        self.value = value

    def perform_on(self, game: board.Board):
        from_square = self[move.from_col][move.from_row]
        to_square = self[move.to_col][move.to_row]

        if from_square.piece == None:
            raise MoveException("This move is illegal because it is from a square without a piece.")
        
        #maybe check if it's a capture?

        to_square.piece = copy.deepcopy(from_square.piece)
        to_square.piece.has_moved = True
        to_square.piece.location = to_square
        from_square.piece = None

    def is_illegal(self, game: board.Board) -> bool:
        return False

    def __str__(self) -> str:
        return f"Move from {self.from_col}{self.from_row} to {self.to_col}{self.to_row} (standard: {self.from_col}{self.from_row + 1} -> {self.to_col}{self.to_row + 1}); value: {self.value}"

    def __repr__(self) -> str:
        return f"Move from {self.from_col}{self.from_row} to {self.to_col}{self.to_row} (standard: {self.from_col}{self.from_row + 1} -> {self.to_col}{self.to_row + 1}); value: {self.value}"


class MoveException(Exception):
    """Exception raised when something goes wrong with a move."""
    
    def __init__(self, message: str):
        super().__init__(message)


def potential_moves(piece: board.Piece | None, game: board.Board) -> list[Move]:
    # make LSP happy
    if piece == None:
        return []
    
    match piece.ptype:
        case board.PieceType.PAWN:
            return pawn_moves(piece, game)
        case board.PieceType.ROOK:
            return rook_moves(piece, game)
        case board.PieceType.KING:
            return king_moves(piece, game)
        case _:
            return []


def pawn_moves(piece: board.Piece, game: board.Board) -> list[Move]:
    direction = 1 if piece.color == board.PieceColor.WHITE else -1
    moves = []
    loc = piece.location
    
    #one square push
    moves.append(Move(loc.col, loc.row, loc.col, loc.row + direction))

    #two square push
    if not piece.has_moved:
        moves.append(Move(loc.col, loc.row, loc.col, loc.row + (direction * 2)))

    #diagonals
    col_num = LETTERS.index(loc.col)
    #left diagonal
    if col_num > 0 and game[LETTERS[col_num - 1]][loc.row + direction].piece != None:
        moves.append(Move(loc.col, loc.row, LETTERS[col_num - 1], loc.row + direction))
    #right diagonal
    if col_num < 7 and game[LETTERS[col_num + 1]][loc.row + direction].piece != None:
        moves.append(Move(loc.col, loc.row, LETTERS[col_num + 1], loc.row + direction))

    return moves


def rook_moves(piece: board.Piece, game: board.Board) -> list[Move]:
    moves = []
    loc = piece.location
    row = loc.row
    col = loc.col
    col_num = LETTERS.index(col)
    
    #move right
    peek_col_num = col_num + 1
    while peek_col_num < 8:
        peek_piece = game[LETTERS[peek_col_num]][row].piece
        temp_move = Move(col, row, LETTERS[peek_col_num], row, peek_piece.ptype.value if peek_piece != None else 0)
        if peek_piece == None:
            moves.append(temp_move)
        elif peek_piece.color != piece.color:
            moves.append(temp_move)
            break
        else:
            break
        peek_col_num += 1

    #move left
    peek_col_num = col_num - 1
    while peek_col_num > -1:
        peek_piece = game[LETTERS[peek_col_num]][row].piece
        temp_move = Move(col, row, LETTERS[peek_col_num], row, peek_piece.ptype.value if peek_piece != None else 0)
        if peek_piece == None:
            moves.append(temp_move)
        elif peek_piece.color != piece.color:
            moves.append(temp_move)
            break
        else:
            break
        peek_col_num -= 1
    
    #move down
    peek_row = row - 1
    while peek_row > -1:
        peek_piece = game[col][peek_row].piece
        temp_move = Move(col, row, col, peek_row, peek_piece.ptype.value if peek_piece != None else 0)
        if peek_piece == None:
            moves.append(temp_move)
        elif peek_piece.color != piece.color:
            moves.append(temp_move)
            break
        else:
            break
        peek_row -= 1

    #move up
    peek_row = row + 1
    while peek_row < 8:
        peek_piece = game[col][peek_row].piece
        temp_move = Move(col, row, col, peek_row, peek_piece.ptype.value if peek_piece != None else 0)
        if peek_piece == None:
            moves.append(temp_move)
        elif peek_piece.color != piece.color:
            moves.append(temp_move)
            break
        else:
            break
        peek_row += 1

    return moves


# Find possible but not necessarily legal king moves
def king_moves(piece: board.Piece, game: board.Board) -> list[Castle | Move] | list[Move]:
    row = piece.location.row
    col = piece.location.col
    col_num = LETTERS.index(col)
    squares = []

    # Local function to make things simpler
    def local_square(local_col_num: int, local_row_num: int):
        king_square(game[LETTERS[local_col_num]][local_row_num], piece.color, squares)

    # left
    if col_num > 0:
        local_square(col_num - 1, row)
    
    # bottom left
    if col_num > 0 and row > 0:
        local_square(col_num - 1, row - 1)

    # top left
    if col_num > 0 and row < 7:
        local_square(col_num - 1, row + 1)

    # top
    if row < 7:
        local_square(col_num, row + 1)

    # top right
    if row < 7 and col_num < 7:
        local_square(col_num + 1, row + 1)

    # right
    if col_num < 7:
        local_square(col_num + 1, row)

    # bottom right
    if col_num < 7 and row > 0:
        local_square(col_num + 1, row - 1)

    # bottom
    if row > 0:
        local_square(col_num, row - 1)

    # insane spaghetti one-liner
    # Castling is returned regardless of legality
    moves = [Move(piece.location.col, piece.location.row, square.col, square.row, value) for (square, value) in squares]
    if not piece.has_moved:
        moves += [Castle(CastleSide.QUEEN, piece.color), Castle(CastleSide.KING, piece.color)]
    return moves


def king_square_value(square: board.Square, king_color: board.PieceColor) -> None | int:
    piece = square.piece
    if piece == None:
        return 0
    if piece.color != king_color:
        return piece.ptype.value
    else:
        return


def king_square(square: board.Square, king_color: board.PieceColor, allowed_squares: list[tuple[board.Square, int]]):
    value = king_square_value(square, king_color)
    if value != None:
        allowed_squares.append((square, value))


LETTERS = "abcdefgh"

if __name__ == "__main__":
    print("src/movement.py: This file is a dependency of other modules in Sophisticate. Running it by itself simply prints this message. To test this file, write a test file in the /testing directory.")

