import testlib
from src.physical import board, movement
from pprint import pprint

game = board.Board()

#illegal moves
testlib.register_function_test(lambda: movement.potential_moves(game["c"][0].piece, game), [], "White c bishop has no legal moves at starting pos")
testlib.register_function_test(lambda: movement.potential_moves(game["c"][7].piece, game), [], "Black c bishop has no legal moves at starting pos")
testlib.register_function_test(lambda: movement.potential_moves(game["d"][0].piece, game), [], "White has no legal queen moves at starting pos")
testlib.register_function_test(lambda: movement.potential_moves(game["d"][7].piece, game), [], "Black has no legal queen moves at starting pos")

#legal moves - with mayhem!
game["c"][4].piece = board.Piece(board.PieceType.QUEEN, board.PieceColor.BLACK, game["c"][4])

