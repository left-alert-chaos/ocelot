# A simple test I made to test testlib and to test the physical representation I have so far.
import testlib
from pprint import pprint
from src import physical

game = physical.board.Board()

print(game["e"][1].piece)
pprint(physical.movement.potential_moves(game["e"][1].piece, game))
pprint(physical.movement.potential_moves(game["e"][6].piece, game))

