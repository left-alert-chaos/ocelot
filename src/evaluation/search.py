"""# search
Module holding logic for search. Evaluates moves by recursing through possible positions.

# Classes

## SearchNode
Class representing a node in a branching search. Holds a game and subseqent nodes.

## SearchTree
Holds a root SearchNode and handles evaluation of many end points.
"""
from physical import movement, board
import copy
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

    ## best_position: physical.board.Board
    The best found position after given depth.

    ## best_value: float
    The best found value after given depth.

    # Methods

    ## __init__(self, game: physical.board.Board, color: physical.board.PieceColor, depth: int)
    Initializes and populates the tree. This is very time complex and can take awhile.

    ## try_best_position(self, position: physical.board.Board, value: float) -> bool
    Checks if value > current best value. If it is, best_position becomes position.
    Returns whether the new position was better.
    """

    def __init__(self, game: board.Board, color: board.PieceColor, depth: int):
        self.game = game
        self.color = color
        self.root = SearchNode(game, color, self)
        self.best_value = 0.0
        self.best_position = game

        self.root.populate(depth)
    
    def try_best_position(self, position: board.Board, value: float):
        if value > self.best_value:
            self.best_value = value
            self.best_position = position


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

    # Methods

    ## __init__(self, game: physical.board.Board, color: physical.board.PieceColor, tree: SearchTree)
    Initializes the node.

    ## populate(self, depth: int) -> list[SearchNode]
    Recursively populates children until given depth.
    Returns children."""

    def __init__(self, game: board.Board, color: board.PieceColor, tree: SearchTree):
        self.color = color
        self.opponent_color = board.PieceColor.WHITE if color == board.PieceColor.BLACK else board.PieceColor.BLACK
        self.game = game
        self.children = []
        self.value = manual.non_predictive(game, color)
        self.tree = tree
    
    def populate(self, depth: int) -> list:
        good = self.tree.try_best_position(self.game, self.value)
        if not good:
            return self.children
        
        moves = movement.white_legal_moves(self.game) if self.color == board.PieceColor.WHITE else movement.black_legal_moves(self.game)
        for move in moves:
            temp_game = copy.deepcopy(self.game)
            move.perform_on(temp_game)
            new_node = SearchNode(temp_game, self.opponent_color, self.tree)
            new_node.populate(depth - 1)
            self.children.append(new_node)

        return self.children

