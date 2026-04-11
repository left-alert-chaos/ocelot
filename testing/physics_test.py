# A simple test I made to test testlib and to test the physical representation I have so far.
import testlib
from pprint import pprint
from src import physical

game = physical.board.Board()

#add a rogue rook
game["e"][3].piece = physical.board.Piece(physical.board.PieceType.ROOK, physical.board.PieceColor.WHITE, game["e"][3])
pprint(physical.movement.potential_moves(game["e"][3].piece, game))
pprint(game["e"][3].piece)

