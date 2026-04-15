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
    move = input("Please enter a castle side (queen or king) or a move stylized as a1 a2:")
    
    move = parse_move(move)
    move.perform_on(game)
    movement.update_threats(game)

def parse_move(move: str) -> movement.Move | movement.Castle:
    words = move.split()
    if len(words) == 2:
        return movement.Move(words[0][0], int(words[0][1]), words[1][0], int(words[1][1]))
    else:
        side = movement.CastleSide.QUEEN if words[0].lower() == "queen" else movement.CastleSide.KING
        return movement.Castle(side, board.PieceColor.BLACK)

move_num = 0
while True:
    move_num += 1
    try:
        make_move()
        user_moves()
    except:
        print(f"End of game. That lasted {move_num} turns!")
        break

