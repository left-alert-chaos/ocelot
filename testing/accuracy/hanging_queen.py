"""Sets up an artificial scenario with a hanging queen. Checks whether the bot takes the queen."""
import testlib
from src.physical import movement, board
from src import bot

game = board.Board(False)
game["a"][0].piece = board.Piece(board.PieceType.KING, board.PieceColor.WHITE, game["a"][0])
game.pieces.append(game["a"][0].piece)
game["c"][0].piece = board.Piece(board.PieceType.KING, board.PieceColor.BLACK, game["c"][0])
game.pieces.append(game["c"][0].piece)
game["c"][1].piece = board.Piece(board.PieceType.QUEEN, board.PieceColor.WHITE, game["c"][1])
game.pieces.append(game["c"][1].piece)

engine = bot.Sophisticate(game, board.PieceColor.BLACK)
testlib.register_function_test(engine.best_move, movement.Move("c", 0, "c", 1), "Black takes hanging queen.")

