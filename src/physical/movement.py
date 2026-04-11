"""Module that holds anything for moves.

# Classes

## Move
Represents a single move on a board.

## MoveException(Exception)
Error for when moves fail.

# Functions

## potential_moves(piece: board.Piece, game: board.Board) -> list[Move]
Finds potential moves for pieces. Does not find only legal moves; doesn't skip pinned pieces and doesn't skip moves that don't end check.

## <piece>_moves(piece: board.Piece, game: board.Board) -> list[Move]
Dependency of potential_moves(). Same shtick, but only for the given piece."""
import board
import copy


class Move:
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
        from_square.piece = None

    def is_illegal(self, game: board.Board) -> bool:
        return False

    def __str__(self) -> str:
        return f"Move from {self.from_col}{self.from_row} to {self.to_col}{self.to_row} (add one to row nums to get standard notation); value: {self.value}"

    def __repr__(self) -> str:
        return f"Move from {self.from_col}{self.from_row} to {self.to_col}{self.to_row} (add one to row nums to get standard notation); value: {self.value}"


class MoveException(Exception):
    """Exception raised when something goes wrong with a move."""
    
    def __init__(self, message: str):
        super().__init__(message)


def potential_moves(piece: board.Piece, game: board.Board) -> list[Move]:
    match piece.ptype:
        case board.PieceType.PAWN:
            return pawn_moves(piece, game)
        case board.PieceType.ROOK:
            return rook_moves(piece, game)
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
    letters = "abcdefgh"
    col_num = letters.index(loc.col)
    #left diagonal
    if col_num > 0 and game[letters[col_num - 1]][loc.row + direction].piece != None:
        moves.append(Move(loc.col, loc.row, letters[col_num - 1], loc.row + direction))
    #right diagonal
    if col_num < 7 and game[letters[col_num + 1]][loc.row + direction].piece != None:
        moves.append(Move(loc.col, loc.row, letters[col_num + 1], loc.row + direction))

    return moves


def rook_moves(piece: board.Piece, game: board.Board) -> list[Move]:
    moves = []
    loc = piece.location
    row = loc.row
    col = loc.col
    letters = "abcdefgh"
    col_num = letters.index(col)
    
    #move right
    peek_col_num = col_num + 1
    while peek_col_num < 8:
        peek_piece = game[letters[peek_col_num]][row].piece
        temp_move = Move(col, row, letters[peek_col_num], row, peek_piece.ptype.value if peek_piece != None else 0)
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
        peek_piece = game[letters[peek_col_num]][row].piece
        temp_move = Move(col, row, letters[peek_col_num], row, peek_piece.ptype.value if peek_piece != None else 0)
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

