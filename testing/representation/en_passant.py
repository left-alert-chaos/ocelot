import testlib
from src.physical import board, movement

game = board.Board()
movement.Move("e", 1, "e", 3, en_passant_vulnerable=True).perform_on(game)
testlib.register_bool_assertion(game["e"][3].piece != None and game["e"][3].piece.en_passant, "e4 is en passant vulnerable")

