"""Module that holds anything for moves and attacks (check, etc).

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

## check_square_value(square: board.Square, player_color: board.PieceColor) -> None | int
Checks if a square has a collision, or is capturable. If None is returned, the square cannot be moved to.
If an integer is returned, it is the value of capturing on that square.

## check_square(square: board.Square, player_color: board.PieceColor, allowed_squares: list[tuple[board.Square, int]])
Runs check_square_value and checks for None. If an integer is returned, the square is appended to allowed_squares.

## project_diagonal(col_change: int, row_change: int, start: board.Square, game: board.Board, color: board.PieceColor, buffer: list[tuple[board.Square, int]])
Iterates until a collision is encountered and puts possible squares in buffer. The col_change and row_change variables are not bound,
so it is technically possible to use this function to find illegal moves.

## update_threats(game: board.Board)
This reads a Board object and populates its threatened_squares and squares_white_threatens and squares_black_threatens attributes.
It isn't a method because of namespacing issues.

## update_<color>_threats(game: board.Board)
Updates threats for only the given color.

## is_check(color: board.PieceColor, game: board.Board) -> bool
Checks if king is threatened and returns True if it is.

## white_legal_moves(game: board.Board) -> list[Move | Castle]
Finds all white moves.

## black_legal_moves(game: board.Board) -> list[Move | Castle]
"""

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
        #incentivise castling
        self.value: int | float = 3

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
            #make the language server happy
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
            #make the language server happy
            if rook == None:
                raise MoveException(f"Somewhere, somehow, the rook on ({game['h'][backrank]}) became None.")
            rook.location.piece = None
            game["f"][backrank].piece = rook
            rook.location = game["f"][backrank]

        game.reset_en_passant
        update_threats(game)


    def is_illegal(self, game: board.Board) -> bool:
        if is_check(self.color, game):
            return True

        rook_col = "h" if self.side == CastleSide.KING else "a"
        backrank = 0 if self.color == board.PieceColor.WHITE else 7
        rook = game[rook_col][backrank].piece

        #check rook legality
        if rook == None or rook.has_moved or rook.color != self.color:
            return True

        #king legality
        king = game["e"][backrank].piece
        if king == None or king.has_moved:
            return True

        #check clear backrank
        if self.side == CastleSide.KING:
            return not (game["f"][backrank].piece == None and game["g"][backrank].piece == None)
        else:
            return not (game["b"][backrank].piece == None and game["c"][backrank].piece == None and game["d"][backrank].piece == None)


    def __str__(self) -> str:
        return f"{self.color.name} castles {self.side.name}side."

    def __repr__(self) -> str:
        return f"{self.color.name} castles {self.side.name}side."

    def __hash__(self) -> int:
        return hash(str(self))


class Move(Action):
    """Simple class to represent a move.

    # Methods

    ## __init__(self, from_col: str, from_row: int, to_col: str, to_row: int, promotion_type: board.PieceType|None = None, en_passant_vulnerable: bool=False, en_passant_square_col: str|None=None, en_passant_square_row: int|None=None)
    Pretty self-explanatory.

    ## perform_on(self, game: board.Board)
    Checks whether a piece is on from square. If there isn't, raises an exception. Else, moves the piece.

    ## is_illegal(self, game: board.Board)
    Checks for illegal move (moving pinned piece, etc) and returns True if illegal is False if legal.
    """

    def __init__(self, from_col: str, from_row: int, to_col: str, to_row: int, value: float=0.0, promotion_type: board.PieceType|None = None, en_passant_vulnerable: bool=False, en_passant_square_row: int|None=None, en_passant_square_col: str|None=None):
        super().__init__()
        self.from_col = from_col
        self.from_row = from_row
        self.to_col = to_col
        self.to_row = to_row
        self.value = value
        self.promotion = promotion_type
        self.en_passant_vulnerable = en_passant_vulnerable
        self.en_passant_square_row = en_passant_square_row
        self.en_passant_square_col = en_passant_square_col

    def perform_on(self, game: board.Board):
        from_square = game[self.from_col][self.from_row]
        to_square = game[self.to_col][self.to_row]

        if from_square.piece == None:
            raise MoveException(f"This move ({self}) is illegal and unplayable because it is from a square without a piece.")
        if to_square.piece != None:
            remove = game.pieces.count(to_square.piece)
            for _ in range(remove):
                game.pieces.remove(to_square.piece)

        #actually move the piece
        to_square.piece = copy.deepcopy(from_square.piece)
        game.pieces.append(to_square.piece)
        to_square.piece.has_moved = True
        to_square.piece.location = to_square
        game.pieces.remove(from_square.piece)
        from_square.piece = None

        #check for promotion
        if self.promotion != None:
            to_square.piece.ptype = self.promotion

        #en passant
        if self.en_passant_square_row != None and self.en_passant_square_col != None:
            square = game[self.en_passant_square_col][self.en_passant_square_row]
            game.pieces.remove(square.piece)
            square.piece = None

        #update board because I'll forget to do it later
        game.reset_en_passant()
        update_threats(game)
        to_square.piece.en_passant = self.en_passant_vulnerable

    def is_illegal(self, game: board.Board) -> bool:
        from_square = game[self.from_col][self.from_row]
        local_board = game.duplicate()
        try:
            self.perform_on(local_board)
        except:
            return True

        if from_square.piece == None:
            print(f"{self} is illegal because it is from a square without a piece.")
            return True

        color = from_square.piece.color
        return is_check(color, local_board)


    def __str__(self) -> str:
        string = f"Move from {self.from_col}{self.from_row} to {self.to_col}{self.to_row} (standard: {self.from_col}{self.from_row + 1} -> {self.to_col}{self.to_row + 1}); value: {self.value}"
        if self.promotion != None:
            string += f"; promotes to {self.promotion}"
        return string

    def __repr__(self) -> str:
        string = f"Move from {self.from_col}{self.from_row} to {self.to_col}{self.to_row} (standard: {self.from_col}{self.from_row + 1} -> {self.to_col}{self.to_row + 1}); value: {self.value}"
        if self.promotion != None:
            string += f"; promotes to {self.promotion}"
        return string
    
    def __hash__(self) -> int:
        return hash(str(self))

    def __eq__(self, other) -> bool:
        if other == None: return False
        return self.to_col == other.to_col and self.to_row == other.to_row and self.from_col == other.from_col and self.from_row == other.from_row

    def __ne__(self, other) -> bool:
        return not self.__eq__(other)


class MoveException(Exception):
    """Exception raised when something goes wrong with a move."""
    
    def __init__(self, message: str):
        super().__init__(message)


def potential_moves(piece: board.Piece | None, game: board.Board) -> list[Castle | Move] | list[Move]:
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
        case board.PieceType.KNIGHT:
            return knight_moves(piece, game)
        case board.PieceType.BISHOP:
            return bishop_moves(piece, game)
        case board.PieceType.QUEEN:
            return rook_moves(piece, game) + bishop_moves(piece, game)


def pawn_moves(piece: board.Piece, game: board.Board) -> list[Move]:
    direction = 1 if piece.color == board.PieceColor.WHITE else -1
    moves = []
    loc = piece.location
    backrank = 7 if direction == 1 else 0
    
    #one square push
    if loc.row < 7 and game[loc.col][loc.row + direction].piece == None:
        promotion_piece = board.PieceType.QUEEN if loc.row + direction == backrank else None
        moves.append(Move(loc.col, loc.row, loc.col, loc.row + direction, promotion_type=promotion_piece))

    #two square push
    if not piece.has_moved:
        #check for collisions on the way
        middle_piece = game[loc.col][loc.row + direction].piece
        last_piece = game[loc.col][loc.row + (direction * 2)].piece
        if middle_piece == None and last_piece == None:
            moves.append(Move(loc.col, loc.row, loc.col, loc.row + (direction * 2), en_passant_vulnerable=True))

    #diagonals
    col_num = LETTERS.index(loc.col)
    #left diagonal
    if col_num > 0 and game[LETTERS[col_num - 1]][loc.row + direction].piece != None and game[LETTERS[col_num - 1]][loc.row + direction].piece.color != piece.color:
        moves.append(Move(loc.col, loc.row, LETTERS[col_num - 1], loc.row + direction))
    #right diagonal
    if col_num < 7 and game[LETTERS[col_num + 1]][loc.row + direction].piece != None and game[LETTERS[col_num + 1]][loc.row + direction].piece.color != piece.color:
        moves.append(Move(loc.col, loc.row, LETTERS[col_num + 1], loc.row + direction))

    #en passant
    if col_num > 0 and game[LETTERS[col_num - 1]][loc.row].piece != None:
        en_passant_square = game[LETTERS[col_num - 1]][loc.row]
        en_passant_piece = en_passant_square.piece

        if en_passant_piece != None:
            if en_passant_piece.ptype == board.PieceType.PAWN and en_passant_piece.en_passant:
                moves.append(Move(loc.col, loc.row, LETTERS[col_num - 1], loc.row + direction, en_passant_square_row=en_passant_square.row, en_passant_square_col=en_passant_square.col, value=1))
    
    if col_num < 7 and game[LETTERS[col_num + 1]][loc.row].piece != None:
        en_passant_square = game[LETTERS[col_num - 1]][loc.row]
        en_passant_piece = en_passant_square.piece

        if en_passant_piece != None:
            if en_passant_piece.ptype == board.PieceType.PAWN and en_passant_piece.en_passant:
                moves.append(Move(loc.col, loc.row, LETTERS[col_num + 1], loc.row + direction, en_passant_square_row=en_passant_square.row, en_passant_square_col=en_passant_square.col, value=1))

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


def knight_moves(piece: board.Piece, game: board.Board) -> list[Move]:
    squares = []
    col = piece.location.col
    col_num = LETTERS.index(col)
    row = piece.location.row

    #local function to make things simpler
    def local_square(local_col_num: int, local_row_num: int):
        check_square(game[LETTERS[local_col_num]][local_row_num], piece.color, squares)

    #right 1 down 2
    if col_num < 7 and row > 1:
        local_square(col_num + 1, row - 2)

    #left 1 down 2
    if col_num > 0 and row > 1:
        local_square(col_num - 1, row - 2)

    #left 2 down 1
    if col_num > 1 and row > 0:
        local_square(col_num - 2, row - 1)

    #left 2 up 1
    if col_num > 1 and row < 7:
        local_square(col_num - 2, row + 1)

    #left 1 up 2
    if col_num > 0 and row < 6:
        local_square(col_num - 1, row + 2)

    #right 1 up 2
    if col_num < 7 and row < 6:
        local_square(col_num + 1, row + 2)

    #right 2 up 1
    if col_num < 6 and row < 7:
        local_square(col_num + 2, row + 1)

    #right 2 down 1
    if col_num < 6 and row > 0:
        local_square(col_num + 2, row - 1)

    #more insane spaghetti
    return [Move(col, row, square.col, square.row, value) for (square, value) in squares]


# Find possible but not necessarily legal king moves
def king_moves(piece: board.Piece, game: board.Board) -> list[Castle | Move] | list[Move]:
    row = piece.location.row
    col = piece.location.col
    col_num = LETTERS.index(col)
    squares = []

    # Local function to make things simpler
    def local_square(local_col_num: int, local_row_num: int):
        check_square(game[LETTERS[local_col_num]][local_row_num], piece.color, squares)

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
    moves = [Move(col, row, square.col, square.row, value) for (square, value) in squares]
    if not piece.has_moved:
        moves += [Castle(CastleSide.QUEEN, piece.color), Castle(CastleSide.KING, piece.color)]
    return moves


def check_square_value(square: board.Square, player_color: board.PieceColor) -> None | int:
    piece = square.piece
    if piece == None:
        return 0
    if piece.color != player_color:
        return piece.ptype.value
    else:
        return


def check_square(square: board.Square, player_color: board.PieceColor, allowed_squares: list[tuple[board.Square, int]]):
    value = check_square_value(square, player_color)
    if value != None:
        allowed_squares.append((square, value))


def bishop_moves(piece: board.Piece, game: board.Board) -> list[Move]:
    squares = []

    #wrap project_diagonal() to be simpler
    def local_diagonal(col_change: int, row_change: int):
        project_diagonal(col_change, row_change, piece.location, game, piece.color, squares)

    #up and left
    local_diagonal(-1, 1)

    #up and right
    local_diagonal(1, 1)

    #down and right
    local_diagonal(1, -1)

    #down and left
    local_diagonal(-1, -1)

    row = piece.location.row
    col = piece.location.col
    return [Move(col, row, square.col, square.row, value) for (square, value) in squares]


def project_diagonal(col_change: int, row_change: int, start: board.Square, game: board.Board, color: board.PieceColor, buffer: list[tuple[board.Square, int]]):
    col_num = LETTERS.index(start.col)
    row = copy.copy(start.row)

    #move once
    col_num += col_change
    row += row_change
    
    #iterate until a collision or end of board
    while col_num >= 0 and row >= 0 and col_num < 8 and row < 8:
        peek_square = game[LETTERS[col_num]][row]
        peek_value = check_square_value(peek_square, color)
        if peek_value == None:
            break
        
        buffer.append((peek_square, peek_value))

        #check if break is necessary
        if peek_value > 0 and peek_value != None:
            break

        #move
        col_num += col_change
        row += row_change


def update_threats(game: board.Board):
    update_white_threats(game)
    update_black_threats(game)

    #tie together
    game.threatened_squares = game.squares_white_threatens + game.squares_black_threatens


def update_white_threats(game: board.Board):
    #same for white
    white_moves = []
    for piece in game.white_pieces():
        white_moves += potential_moves(piece, game)
    #remove dupes
    game.squares_white_threatens = list(set([game[move.to_col][move.to_row] for move in white_moves if not isinstance(move, Castle)]))


def update_black_threats(game: board.Board):
    #get potential moves and extract to_col and to_row attributes
    black_moves = []
    for piece in game.black_pieces():
        black_moves += potential_moves(piece, game)
    #remove duplicates with list(set)
    game.squares_black_threatens = list({game[move.to_col][move.to_row] for move in black_moves if not isinstance(move, Castle)})


def is_check(color: board.PieceColor, game: board.Board) -> bool:
    pieces = game.white_pieces() if color == board.PieceColor.WHITE else game.black_pieces()
    enemy_threats = game.squares_black_threatens if color == board.PieceColor.WHITE else game.squares_white_threatens
    king = None
    for piece in pieces:
        if piece.ptype == board.PieceType.KING:
            king = piece
            break
    if king == None:
        print(f"WARNING: Not check because there is no {color.name} king!")
        return False

    return king.location in enemy_threats

        
class PieceException(Exception):
    def __init__(self, message: str):
        super().__init__(message)


def white_legal_moves(game: board.Board) -> list[Move | Castle]:
    moves = []
    for piece in game.white_pieces():
        moves += potential_moves(piece, game)

    #crudely order moves
    moves = [move for move in moves if not move.is_illegal(game)]
    captures = []
    non_captures = []
    for move in moves:
        if isinstance(move, Castle) or game[move.to_col][move.to_row].piece == None:
            non_captures.append(move)
        else:
            captures.append(move)
    return captures + non_captures


def black_legal_moves(game: board.Board) -> list[Move | Castle]:
    moves = []
    for piece in game.black_pieces():
        moves += potential_moves(piece, game)

    #crudely order moves
    moves = [move for move in moves if not move.is_illegal(game)]
    captures = []
    non_captures = []
    for move in moves:
        if isinstance(move, Castle) or game[move.to_col][move.to_row].piece == None:
            non_captures.append(move)
        else:
            captures.append(move)
    return captures + non_captures


LETTERS = "abcdefgh"

if __name__ == "__main__":
    print("src/movement.py: This file is a dependency of other modules in Sophisticate. Running it by itself simply prints this message. To test this file, write a test file in the /testing directory. Docstrings should be ample documentation.")

