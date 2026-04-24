import testlib
from src.physical import board, movement

game = board.Board(False)
game["e"][0].piece = board.Piece(board.PieceType.KING, board.PieceColor.WHITE, game["e"][0])
game["e"][7].piece = board.Piece(board.PieceType.KING, board.PieceColor.BLACK, game["e"][7])
game["h"][6].piece = board.Piece(board.PieceType.PAWN, board.PieceColor.WHITE, game["h"][6])
game.pieces.append(game["e"][0].piece)
game.pieces.append(game["e"][7].piece)
game.pieces.append(game["h"][6].piece)
movement.Move("h", 6, "h", 7, promotion_type=board.PieceType.QUEEN).perform_on(game)
testlib.register_bool_assertion(game["h"][7].piece.ptype == board.PieceType.QUEEN, "Promotion successful")

