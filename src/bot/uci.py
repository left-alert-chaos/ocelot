"""# uci
A simple implementation of the UCI protocol.

# Classes

## UCIException(Exception)
An exception to raise when UCI parsing/generation goes wrong.

# Functions

## create_action(uci_code: str, game: physical.board.Board) -> physical.movement.Action
Takes a move code (like e2e4) and returns an action (Castle or Move)

## run_command(command: str, engine: robot.Sophisticate) -> bool
Parses a command and uses provided engine to fulfill it.
Returns whether to keep the engine running."""

from physical import board, movement
from physical.board import PieceColor, PieceType
import evaluation
from robot import Sophisticate


def create_action(uci_code: str, game: board.Board) -> movement.Action:
    if not len(uci_code) >= 4:
        raise UCIException(f"Couldn't parse uci code '{uci_code}' because it is less than 4 characters.")

    col1 = uci_code[0]
    row1 = int(uci_code[1]) - 1
    col2 = uci_code[2]
    row2 = int(uci_code[3]) - 1
    move = movement.Move(col1, row1, col2, row2)

    #return a Castle if it's a king and it moves more than 1 square
    from_piece = game[col1][row1].piece
    if from_piece != None and from_piece.ptype == PieceType.KING:
        #find how many rows were moved
        distance = abs(row1 - row2)
        
        if distance > 1:
            return movement.Castle(movement.CastleSide.KING if distance == 2 else movement.CastleSide.QUEEN, from_piece.color)

    if len(uci_code) > 4:
        match uci_code[4].lower():
            case "n":
                move.promotion = PieceType.KNIGHT
            case "b":
                move.promotion = PieceType.BISHOP
            case "r":
                move.promotion = PieceType.ROOK
            case "q":
                move.promotion = PieceType.QUEEN
    return move


def run_command(command: str, engine: Sophisticate) -> bool:
    command = command.strip()
    words = command.split()
    if len(words) == 0:
        raise UCIException("UCI command is empty.")
    keyword = words[0]

    #implement all mandatory UCI commands that need responses
    #A lot of commands just aren't implemented, so they don't do anything but make the engine smile and nod.
    match keyword:
        case "quit":
            return False
        case "uci":
            print("uciok")
        case "isready":
            print("readyok")
        case "go":
            move = engine.best_move()
            move.perform_on(engine.game)
            #Sophisticate can't ponder, so it just makes a joke
            print(f"bestmove {move.uci()} ponder noclu")
    return True


class UCIException(Exception):
    """# UCIException(Exception)
    An exception to use when UCI parsing/generation goes wrong.

    # Methods

    ## __init__(self, message: str)
    Initializes the exception.
    """

    def __init__(self, message: str):
        self.message = message
        super().__init__(message)

