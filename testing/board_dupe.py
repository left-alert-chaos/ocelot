import os, sys
sys.path.append(os.path.dirname(os.path.dirname(__file__)))
from src.physical import board, movement
import testlib

game = board.Board()
movement.Move("e", 1, "e", 3).perform_on(game)

testlib.register_bool_assertion(game == game.duplicate(), "Duplication works")

