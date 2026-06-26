"""# search
Module holding logic for search. Evaluates moves by recursing through possible positions.

# Classes

## SearchNode
Class representing a node in a branching search. Holds a game and subseqent nodes.

## SearchTree
Holds a root SearchNode and handles evaluation of many end points.
"""
from physical import movement, board
import manual


class SearchTree:
    """# SearchTree
    Holds a root SearchNode and evaluates all end points to find best current move.

    # Attributes

    ## root: SearchNode
    The root node.

    ## game: physical.board.Board
    The game to evaluate.

    ## color: physical.board.PieceColor
    The person whose turn it is.

    ## best_value: float
    The best found value after given depth.

    # Methods

    ## __init__(self, game: physical.board.Board, color: physical.board.PieceColor, depth: int)
    Initializes the tree.

    ## try_best_position(self, position: SearchNode) -> bool
    Checks if value > current best value. If it is, best_position becomes position.
    Returns whether the new position was better or equal.

    ## best_move(self) -> movement.Move | movement.Castle | None
    Populates the tree. This is very time complex and can take a while.
    It finds the best position and returns the move leading to it.
    A minimax algorithm is used with alpha-beta pruning.
    """

    def __init__(self, game: board.Board, color: board.PieceColor, depth: int):
        self.game = game
        self.color = color
        self.positions = 0
        self.root = SearchNode(game, color, self, self)
        self.depth = depth
 
    def best_move(self) -> movement.Action | None:
        self.root.alphabeta(self.depth, float("-inf"), float("inf"), True)

        if len(self.root.children) == 0:
            print("SearchTree.best_move(): self.root.children's len is 0")
            return

        #play favorites as a placeholder, haha
        best_move = None
        best_value = self.root.children[0].value

        for (move, position) in self.root.move_results.items():
            if best_move == None:
                best_move = move
                continue
            if position.value >= best_value:
                best_value = position.value
                best_move = move
            
        if best_move != None: return best_move

        print("SearchTree.best_move(): Couldn't find position; returning None")
        return


class SearchNode:
    """# SearchNode
    Class representing a node in a branching search. Holds a game and subsequent nodes.

    # Attributes

    ## children: list[SearchNode]
    Subsequent positions.

    ## game: physical.board.Board
    The board this node holds.

    ## color: physical.board.PieceColor
    The person whose turn it is.

    ## value: float
    Evaluation of the current board.

    ## tree: SearchTree
    The tree the node is a part of.

    ## move_results: dict[movement.Move, SearchNode]
    A dictionary of moves to the resulting nodes.

    # Methods

    ## __init__(self, game: physical.board.Board, color: physical.board.PieceColor, tree: SearchTree, parent: SearchNode)
    Initializes the node.

    ## populate(self, depth: int) -> bool
    Recursively populates children until given depth.
    Returns whether to continue at current depth or skip."""

    def __init__(self, game: board.Board, color: board.PieceColor, tree: SearchTree, parent):
        self.color = color
        self.opponent_color = board.PieceColor.WHITE if color == board.PieceColor.BLACK else board.PieceColor.BLACK
        self.game = game
        self.children = []
        self.move_results = {}
        self.value = manual.non_predictive(game, color)
        self.tree = tree
        self.tree.positions += 1
        self.parent = parent
    
    #brazenly stolen from https://en.wikipedia.org/wiki/Alpha%E2%80%93beta_pruning#Pseudocode
    def alphabeta(self, depth: int, alpha: float | int, beta: float | int, maximizing_player: bool) -> float:
        #end of game or tree
        if depth == 0 or self.value == float("inf") or self.value == float("-inf"):
            return self.value * (1 if maximizing_player else -1)

        moves = movement.white_legal_moves(self.game) if self.color == board.PieceColor.WHITE else movement.black_legal_moves(self.game)

        if maximizing_player:
            value = float("-inf")
            for move in moves:
                #create child
                temp_game = self.game.duplicate()
                move.perform_on(temp_game)
                child = SearchNode(temp_game, self.opponent_color, self.tree, self)
                self.children.append(child)
                move.value = child.value
                self.move_results[move] = child

                #pruning
                value = max(value, child.alphabeta(depth - 1, alpha, beta, False))
                if value >= beta:
                    break
                alpha = max(alpha, value)
        else:
            value = float("inf")
            for move in moves:
                #create child
                temp_game = self.game.duplicate()
                move.perform_on(temp_game)
                child = SearchNode(temp_game, self.opponent_color, self.tree, self)
                self.children.append(child)
                self.move_results[move] = child

                #pruning
                value = min(value, child.alphabeta(depth - 1, alpha, beta, True))
                if value <= alpha:
                    break
                beta = min(beta, value)
            #Not sure if this mattered, so I'll keep it commented out
            #value *= -1
        self.value = value
        return value


    def __eq__(self, other) -> bool:
        return self.game == other.game
    
    def __ne__(self, other) -> bool:
        return self.game != other.game

