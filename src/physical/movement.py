import board
import copy


class Move:
    """Simple class to represent a move.

    # Methods

    __init__(self, from_col: str, from_row: int, to_col: str, to_row: int)
    Pretty self-explanatory.

    perform_on(self, game: board.Board)
    Checks whether a piece is on from square. If there isn't, raises and exception. Else, moves the piece."""

    def __init__(self, from_col: str, from_row: int, to_col: str, to_row: int):
        self.from_col = from_col
        self.from_row = from_row
        self.to_col = to_col
        self.to_row = to_row

    def perform_on(self, game: board.Board):
        from_square = self[move.from_col][move.from_row]
        to_square = self[move.to_col][move.to_row]

        if from_square.piece == None:
            raise MoveException("This move is illegal because it is from a square without a piece.")
        
        #maybe check if it's a capture?

        to_square.piece = copy.deepcopy(from_square.piece)
        from_square.piece = None


class MoveException(Exception):
    """Exception raised when something goes wrong with a move."""
    
    def __init__(self, message: str):
        super().__init__(message)

