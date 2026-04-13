"""An absolutely bogus position used for testing check"""

import testlib
from src.physical import board, movement

#first test setup
game = board.Board(default_pos=False)
game["a"][0].piece = board.Piece(board.PieceType.QUEEN, board.PieceColor.WHITE, game["a"][0])
game["b"][0].piece = board.Piece(board.PieceType.KING, board.PieceColor.BLACK, game["b"][0])
game.pieces.append(game["a"][0].piece)
game.pieces.append(game["b"][0].piece)
movement.update_threats(game)

testlib.register_function_test(lambda: movement.is_check(board.PieceColor.BLACK, game), True, "Queen next to king is check")

default_game = board.Board()
testlib.register_function_test(lambda: movement.is_check(board.PieceColor.BLACK, default_game), False, "Default position isn't check")

