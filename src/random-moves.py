"""A sample implementation that randomly chooses legal moves. More a low-effort, unstable tech demo than anything else. Castling isn't supported."""

from physical import board, movement
import random

game = board.Board()

def make_move():
    global game
    if movement.is_check(board.PieceColor.WHITE, game):
        print("I'm in check!!!")
    potential = []
    for piece in game.white_pieces():
        potential += movement.potential_moves(piece, game)
    moves = []

    for move in potential:
        if move.is_illegal(game):
            continue
        piece = game[move.from_col][move.from_row].piece
        if piece == None:
            continue
        if move.to_row == 7 and piece.ptype == board.PieceType.PAWN:
            move.promotion = board.PieceType.QUEEN
        moves.append(move)

    move = random.choice(moves)
    move.perform_on(game)
    movement.update_threats(game)
    print(move)

def user_moves():
    global game
    move = input("Please enter a castle side (queen or king) or a move stylized as a1 a2 <promotion_type> (start with 'ep ' to en passant): ")
    
    move = parse_move(move)
    move.perform_on(game)
    movement.update_threats(game)

def parse_move(move: str) -> movement.Move | movement.Castle:
    global game
    en_passant = True if move.startswith("ep ") else False
    words = move.split()
    if len(words) > 1:
        f = words[0]
        to = words[1]
        promote = None

        #promotion
        if len(words) == 3:
            match words[2].lower():
                case "queen":
                    promote = board.PieceType.QUEEN
                case "knight":
                    promote = board.PieceType.KNIGHT
                case "bishop":
                    promote = board.PieceType.BISHOP
                case "rook":
                    promote = board.PieceType.ROOK
                case _:
                    print("Couldn't figure out what piece to promote to, so defaulting to None.")

        return movement.Move(f[0], int(f[1]) - 1, to[0], int(to[1]) - 1, promotion_type=promote, en_passant_square_col=to[0], en_passant_square_row=int(to[1]) + 1 if en_passant else None)
    else:
        side = movement.CastleSide.QUEEN if words[0].lower() == "queen" else movement.CastleSide.KING
        return movement.Castle(side, board.PieceColor.BLACK)

move_num = 0
while True:
    move_num += 1
    try:
        make_move()
        user_moves()
    except Exception as e:
        print(f"End of game. That lasted {move_num} turns!")
        print(f"error: {e}")
        break

