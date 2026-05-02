"""Module holding Board and Move classes.
All row values are 0-indexed,
so no matter what part of the engine you're in (unless it's user-facing),
row 7 is the equivalent of real-life row 8.

# Functions

## col_row(square: Square) -> tuple[str, int]
Returns the info of a square.

# Classes

## Square
Represents a square on the board.

## PieceColor
Enum for color of a player's pieces. Also used in other parts of the engine to distinguish players.

## PieceType
Enum for type of pieces, including value.

## Piece
A piece on the board, holding color, type, and location.

## Board
The board itself. Holds columns, rows, and sets up starting position by default.
"""
from enum import Enum
import copy
import time

#used to track how long duplication has taken
elapsed_duplication = 0.0


class Square:
    """# Square
    One square on the board.

    # Methods

    ## __init__(self, col: str, row: int)
    Col is a regular board column (a - h). Row is a zero-indexed row. So, what GothamChess would call H8 would be Square(\"h\", 7).

    ## draw(self) -> str
    Returns very bad ascii art of the square."""

    def __init__(self, col: str, row: int):
        self.col = col
        self.row = row
        self.piece: Piece | None = None

        #a very complicated-looking way to determine square color.
        #If the row number plus the column number are even, black. else white
        self.color = PieceColor.BLACK if (LETTERS.index(col) + row) % 2 == 0 else PieceColor.WHITE

    def __str__(self) -> str:
        return f"Square {self.col}{self.row} (0-indexed; {self.col}{self.row + 1} in standard notation)"

    def __repr__(self) -> str:
        return f"Square {self.col}{self.row} (0-indexed; {self.col}{self.row + 1} in standard notation)"

    def __eq__(self, other) -> bool:
        return self.piece == other.piece and self.col == other.col and self.row == other.row
    
    def __ne__(self, other) -> bool:
        return not self.__eq__(other)

    def __hash__(self) -> int:
        #be laaaaaaazy
        return hash(str(self))

    def draw(self) -> str:
        filler = "bb" if self.color == PieceColor.BLACK else "ww"
        piece_repr = "  " if self.piece == None else self.piece.two_letter()

        return f"{filler}{piece_repr}{filler}"


def col_row(square: Square) -> tuple[str, int]:
    return (square.col, square.row)


class PieceColor(Enum):
    """Enum representing colors. BLACK is 0 and WHITE is 1. Also represents board square colors."""
    BLACK = 0
    WHITE = 1


class PieceType(Enum):
    """Enum representing types (Bishop, Queen, King, etc).

    # Values

    PAWN = 1
    KNIGHT = 3
    BISHOP = 4 because I personally like bishops more than knights
    ROOK = 5
    QUEEN = 9
    KING = 39 because all other  together are 39 and the king is worth the game.
    """
    PAWN = 1
    KNIGHT = 3
    BISHOP = 4
    ROOK = 5
    QUEEN = 9
    KING = 39


class Piece:
    """Class representing a on the 

    # Methods

    ## __init__(self, ptype: PieceType, color: PieceColor, location: Square)
    Pretty self-explanatory.

    ## two_letter(self) -> str
    Returns 2-letter string of piece: 'BQ' for black queen, 'WK' for white king, and so on.

    # Attributes

    ptype: PieceType - The type of the piece
    color: PieceColor - The color of the piece
    location: Square - The square the is on
    en_passant: bool=False - Whether the piece can be captured with en passant.
    """
    
    def __init__(self, ptype: PieceType, color: PieceColor, location: Square):
        self.ptype = ptype
        self.color = color
        self.location = location
        self.has_moved = False
        self.en_passant = False

    def __str__(self) -> str:
        return f"{self.color.name} {self.ptype.name} on {self.location}"

    def __repr__(self) -> str:
        return f"{self.color.name} {self.ptype.name} on {self.location}"

    def __eq__(self, other) -> bool:
        if other == None: return False
        return self.ptype == other.ptype and self.color == other.color

    def __ne__(self, other) -> bool:
        return not self.__eq__(other)

    def two_letter(self) -> str:
        #                                                knights should be represented with n instead of k
        return f"{self.color.name[0]}{self.ptype.name[0] if self.ptype.name != 'KNIGHT' else 'N'}"


class Board:
    """Class to represent a chess board.
    Automatically populates with 8 lettered columns of 8 squares.

    # Attributes

    ## threatened_squares: list[Square]
    All squares on the board that can be moved to by any player.

    ## squares_white_threatens: list[Square]
    All squares on the board that can be moved to by white.

    ## squares_black_threatens: list[Square]
    All squares on the board that can be moved to by black.

    ## pieces: list[Piece]
    All pieces on the board.

    ## white_castled: bool=False
    Has white castled?

    ## black_castled: bool=False
    Has black castled?
    
    # Methods

    ## __init__(self, default_pos: bool=True)
    If default_pos is True, a stock starting position is automatically set up.

    ## __getitem__(self, col_name: str) -> list[Square]
    Takes a column name and returns a list of squares.

    ## white_pieces(self) -> list[Piece]
    Iterates through pieces and returns white pieces in no particular order.

    ## black_pieces(self) -> list[Piece]
    Iterates through pieces and returns white pieces in no particular order.

    ## set_up_game_board(self)
    Puts pieces in default positions for a standard game.

    ## prep_column(self, col_name: str, ptype: PieceType)
    Adds pawns and given piece in correct places to given column.

    ## add_pawns(self, col: list[Square])
    Adds pawns to second and second-to-last squares in column.

    ## reset_en_passant(self)
    Sets all pieces' en_pssant attributes to False.

    ## duplicate(self) -> Board
    Creates a new board identical to this one.

    ## draw(self) -> str
    Draws an ascii representation of the board. The default for __str__ and __repr__.
    """

    def __init__(self, default_pos: bool=True):
        # Set up 8 * 8 board with dict to represent lettered columns
        self.squares: dict[str, list[Square]] = {}
        self.pieces = []
        self.threatened_squares = []
        self.squares_white_threatens = []
        self.squares_black_threatens = []
        self.white_castled, self.black_castled = False, False
        for col in "abcdefgh":
            col_list = []
            for row in range(8):
                col_list.append(Square(col, row))
            self.squares[col] = col_list

        if default_pos:
            self.set_up_game_board()

    def __getitem__(self, col_name: str) -> list[Square]:
        if not isinstance(col_name, str) or len(col_name) != 1 or col_name.isdecimal():
            raise TypeError("col_name is not a single-letter string.")
        return self.squares[col_name]

    def __str__(self) -> str:
        return self.draw()
    
    def __repr__(self) -> str:
        return self.draw()

    def __eq__(self, other) -> bool:
        for col_name in LETTERS:
            for row in range(8):
                if self[col_name][row] != other[col_name][row]: return False
        return True

    def __ne__(self, other) -> bool:
        return not self.__eq__(other)

    def black_pieces(self) -> list[Piece]:
        return [piece for piece in self.pieces if piece.color == PieceColor.BLACK]
    
    def white_pieces(self) -> list[Piece]:
        return [piece for piece in self.pieces if piece.color == PieceColor.WHITE]

    def set_up_game_board(self):
        self.prep_column("a", PieceType.ROOK)
        self.prep_column("h", PieceType.ROOK)
        self.prep_column("b", PieceType.KNIGHT)
        self.prep_column("g", PieceType.KNIGHT)
        self.prep_column("c", PieceType.BISHOP)
        self.prep_column("f", PieceType.BISHOP)
        self.prep_column("d", PieceType.QUEEN)
        self.prep_column("e", PieceType.KING)

    def prep_column(self, col_name: str, ptype: PieceType):
        col = self[col_name]
        col[0].piece = Piece(ptype, PieceColor.WHITE, col[0])
        col[7].piece = Piece(ptype, PieceColor.BLACK, col[7])
        self.pieces.append(col[0].piece)       
        self.pieces.append(col[7].piece)
        self.add_pawns(col)

    def add_pawns(self, col: list[Square]):
        col[1].piece = Piece(PieceType.PAWN, PieceColor.WHITE, col[1])
        col[6].piece = Piece(PieceType.PAWN, PieceColor.BLACK, col[6])
        self.pieces.append(col[1].piece)
        self.pieces.append(col[6].piece)

    def reset_en_passant(self):
        for piece in self.pieces:
            piece.en_passant = False
    
    def duplicate(self):
        global elapsed_duplication
        start = time.time()
        new = Board(False)

        #iterate over columns and rows
        for col in LETTERS:
            for row in range(8):
                piece = self[col][row].piece
                if piece == None:
                    continue

                #copy piece
                new_piece = Piece(copy.deepcopy(piece.ptype), copy.deepcopy(piece.color), new[col][row])
                new.pieces.append(new_piece)
                new[col][row].piece = new_piece
        elapsed_duplication += time.time() - start
        return new

    def draw(self) -> str:
        output = ""
        #all row values, top to bottom
        for row in range(7, -1, -1):
            output += f"\n\n{row + 1}"
            for column in LETTERS:
                output += " | "
                square = self[column][row]
                output += square.draw()

        #Skip first 2 newlines
        return output[2::]


LETTERS = "abcdefgh"

