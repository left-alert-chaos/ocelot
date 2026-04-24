from src.physical import board, movement

game = board.Board()
print("White's b knight's legal opening moves (should be a2 and c2):")
print(movement.potential_moves(game["b"][0].piece, game))
print("Black's b knight's legal opening moves (should be a5 and c5):")
print(movement.potential_moves(game["b"][7].piece, game))

