"""Another tech demo, this time pitting one sophisticate against another."""
import bot
from physical import board
import time

game = board.Board()
white = bot.Sophisticate(game, board.PieceColor.WHITE)
black = bot.Sophisticate(game, board.PieceColor.BLACK)

while True:
    white_move = white.best_move()
    print(f"White's move: {white_move}")
    white_move.perform_on(game)
    input("Press enter for black's move.")
    black_move = black.best_move()
    print(f"Black's move: {black_move}")
    black_move.perform_on(game)
    input("Press enter for white's move.")

