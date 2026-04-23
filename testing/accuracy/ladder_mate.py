import testlib
from src.physical import board, movement
from src import bot

game = board.Board(False)
game["a"][1].piece = board.Piece(board.PieceType.ROOK, board.PieceColor.BLACK, game["a"][1])
game["h"][1].piece = board.Piece(board.PieceType.ROOK, board.PieceColor.BLACK, game["h"][1])
game["a"][7].piece = board.Piece(board.PieceType.KING, board.PieceColor.BLACK, game["a"][7])
game["d"][0].piece = board.Piece(board.PieceType.KING, board.PieceColor.WHITE, game["d"][0])
game.pieces.append(game["a"][1].piece)
game.pieces.append(game["h"][1].piece)
game.pieces.append(game["d"][0].piece)
game.pieces.append(game["a"][7].piece)
engine = bot.Sophisticate(game, board.PieceColor.BLACK)
best = engine.best_move()
print(f"Best move while threatening ladder mate is {best}")
testlib.register_bool_assertion(best == movement.Move("a", 1, "a", 0) or best == movement.Move("h", 1, "h", 0), "Ladder mate with no irrelevant pieces") 

