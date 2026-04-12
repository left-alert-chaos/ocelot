# A simple test I made to test testlib and to test the physical representation I have so far.
import testlib
from pprint import pprint
from src import physical

game = physical.board.Board()

testlib.register_bool_assertion(len(physical.movement.potential_moves(game["e"][0].piece, game)) == 2, "Only castling white king moves")
testlib.register_bool_assertion(len(physical.movement.potential_moves(game["e"][7].piece, game)) == 2, "Only castling in black king moves")

