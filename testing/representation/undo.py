"""# undo
A small test suite for checking whether the new unperform_on(self, game) function works as expected."""

import testlib
from src.physical import board, movement

game = board.Board()

#play e4 (zero-indexed) and undo. check for changes.
move = movement.Move("e", 1, "e", 3)
move.perform_on(game)
move.unperform_on(game)
testlib.register_bool_assertion(game == board.Board(), "Undone e4 results in stock board")

