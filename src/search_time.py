import evaluation
import time
from physical import board

print("Starting time test for depth 5 search from default pos.")
game = board.Board()
print("Starting.")
start = time.time()
tree = evaluation.search.SearchTree(game, board.PieceColor.WHITE, 5)
print(f"Done! Time: {time.time() - start}")
print(f"Best pos is default: {tree.best_position.game == board.Board()}")
print(f"Best value: {tree.best_value}")
print(f"Best move: {tree.best_move()}")
