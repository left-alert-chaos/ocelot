import testlib
from src import bot, physical

game = physical.board.Board()
engine = bot.Sophisticate(game, physical.board.PieceColor.WHITE)
testlib.register_speed_test(engine.best_move, 10, "Start game with white in less than 10 seconds")

