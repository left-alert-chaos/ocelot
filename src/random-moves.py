"""Randomly chooses legal moves. More a low-effort, unstable tech demo than anything else. Castling isn't supported."""
from physical import board, movement
import random
from pprint import pprint

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
    row1 = int(input("Row you're moving from: ")) - 1
    col2 = input("Column you're moving to: ")
    row2 = int(input("Row you're moving to: ")) - 1
    move = movement.Move(col1, row1, col2, row2)
    move.perform_on(game)
    movement.update_threats(game)

move_num = 0
while True:
    move_num += 1
    try:
        make_move()
        user_moves()
    except:
        print(f"End of game. That lasted {move_num} moves!")
        break
