import piece


class Board:
    """Class to represent a chess board.
    Automatically populates with 8 lettered columns of 8 squares.
    
    Methods

    __init__(self, default_pos: bool=True)
    If default_pos is True, a stock starting position is automatically set up."""

    def __init__(self, default_pos: bool=True):
        self.squares = {}
        for col in "abcdefgh":
            col_list = []
            for row in range(8):
                col_list.append(Square(col, row))
            self.squares[col] = col_list


class Square:
    """One square on the board.

    Methods

    __init__(self, col: str, row: int)
    Col is a regular board column (a - h). Row is a zero-indexed row. So, what GothamChess would call H8 would be Square(\"h\", 7)."""

    def __init__(self, col: str, row: int):
        self.col = col
        self.row = row
        self.piece: piece.Piece | None

