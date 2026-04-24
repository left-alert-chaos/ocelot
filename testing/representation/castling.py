import testlib
from src.physical import board, movement

game = board.Board()
movement.Castle(movement.CastleSide.KING, board.PieceColor.WHITE).perform_on(game)
king = game["g"][0].piece
testlib.register_bool_assertion(king != None and king.ptype == board.PieceType.KING and king.color == board.PieceColor.WHITE, "White king in right place")
rook = game["f"][0].piece
testlib.register_bool_assertion(rook != None and rook.ptype == board.PieceType.ROOK and rook.color == board.PieceColor.WHITE, "White rook in right place")

game2 = board.Board()
movement.Castle(movement.CastleSide.QUEEN, board.PieceColor.WHITE).perform_on(game2)
king = game2["c"][0].piece
testlib.register_bool_assertion(king != None and king.ptype == board.PieceType.KING and king.color == board.PieceColor.WHITE, "White king in right place (queenside)")
rook = game2["d"][0].piece
testlib.register_bool_assertion(rook != None and rook.ptype == board.PieceType.ROOK and rook.color == board.PieceColor.WHITE, "White rook in right place (queenside)")

game3 = board.Board()
movement.Castle(movement.CastleSide.KING, board.PieceColor.BLACK).perform_on(game3)
king = game3["g"][7].piece
testlib.register_bool_assertion(king != None and king.ptype == board.PieceType.KING and king.color == board.PieceColor.BLACK, "Black king in right place")
rook = game3["f"][7].piece
testlib.register_bool_assertion(rook != None and rook.ptype == board.PieceType.ROOK and rook.color == board.PieceColor.BLACK, "Black rook in right place")

game4 = board.Board()
movement.Castle(movement.CastleSide.QUEEN, board.PieceColor.BLACK).perform_on(game4)
king = game4["c"][7].piece
testlib.register_bool_assertion(king != None and king.ptype == board.PieceType.KING and king.color == board.PieceColor.BLACK, "Black king in right place (queenside)")
rook = game4["d"][7].piece
testlib.register_bool_assertion(rook != None and rook.ptype == board.PieceType.ROOK and rook.color == board.PieceColor.BLACK, "Black rook in right place (queenside)")

game5 = board.Board()
testlib.register_function_test(lambda: movement.Castle(movement.CastleSide.KING, board.PieceColor.WHITE).is_illegal(game5), True, "White can't castle kingside in default pos")

game6 = board.Board()
testlib.register_function_test(lambda: movement.Castle(movement.CastleSide.KING, board.PieceColor.BLACK).is_illegal(game5), True, "Black can't castle kingside in default pos")

