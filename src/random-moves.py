"""Randomly chooses legal moves. More a low-effort, unstable tech demo than anything else. Castling isn't supported."""
from physical import board, movement
import random

game = board.Board()

def make_move():
    global game
    moves = []
    for piece in game.white_pieces():
        moves += movement.potential_moves(piece, game)
    moves = [move for move in moves if (not isinstance(move, movement.Castle)) and (not move.is_illegal(game))]
    move = random.choice(moves)
    move.perform_on(game)
    movement.update_threats(game)
    print(move)

def user_moves():
    global game
    col1 = input("Column you're moving from: ")
    row1 = int(input("Row you're moving from (0-indexed): "))
    col2 = input("Column you're moving to: ")
    row2 = int(input("Row you're moving to (0-indexed): "))
    move = movement.Move(col1, row1, col2, row2)
    move.perform_on(game)
    movement.update_threats(game)

while True:
    make_move()
    user_moves()

