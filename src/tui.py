"""A sample implementation that uses the bot class. More a low-effort, unstable tech demo than anything else."""

from physical import board, movement
import bot

game = board.Board()
choice = input("Please enter a depth to evaluate (3 is kinda slow, but decent play): ")
depth = int(choice) if choice.isnumeric() else 3
robot = bot.Sophisticate(game, board.PieceColor.WHITE, depth)

def make_move():
    global game
    global robot
    if movement.is_check(board.PieceColor.WHITE, game):
        print("I'm in check!!!")
    
    move = robot.best_move()
    move.perform_on(game)
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
    print(f"Move {move_num}")
    make_move()
    user_moves() 

