import testlib
from src.physical import board, movement
from pprint import pprint

game = board.Board()
movement.Castle(movement.CastleSide.KING, board.PieceColor.WHITE).perform_on(game)
print("WHITE KINGSIDE CASTLE:")
print("piece on g0 (should be white king):")
print(game["g"][0].piece)
print("piece on f0 (should be white rook):")
print(game["f"][0].piece)

print("\n\n")
game2 = board.Board()
movement.Castle(movement.CastleSide.QUEEN, board.PieceColor.WHITE).perform_on(game2)
print("WHITE QUEENSIDE CASTLE:")
print("Piece on c0 (should be white king):")
print(game2["c"][0].piece)
print("Piece on d0 (should be white rook):")
print(game2["d"][0].piece)

print("\n\n")
game3 = board.Board()
movement.Castle(movement.CastleSide.KING, board.PieceColor.BLACK).perform_on(game3)
print("BLACK KINGSIDE CASTLE:")
print("Piece on g7 (should be black king):")
print(game3["g"][7].piece)
print("Piece on f7 (should be black rook):")
print(game3["f"][7].piece)

print("\n\n")
game4 = board.Board()
movement.Castle(movement.CastleSide.QUEEN, board.PieceColor.BLACK).perform_on(game4)
print("BLACK QUEENSIDE CASTLE:")
print("Piece on c7 (should be black king):")
print(game4["c"][7].piece)
print("Piece on d7 (should be black rook):")
print(game4["d"][7].piece)

