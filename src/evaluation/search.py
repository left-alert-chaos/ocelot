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
import random


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

    ## try_best_position(self, position: SearchNode) -> bool
    Checks if value > current best value. If it is, best_position becomes position.
    Returns whether the new position was better or equal.

    ## best_move(self) -> movement.Move
    Traverses parents of best position until a depth 1 move is reached.
    """

    def __init__(self, game: board.Board, color: board.PieceColor, depth: int):
        self.game = game
        self.color = color
        self.root = SearchNode(game, color, self, self)
        self.best_value = 0.0
        self.best_position = self.root

        self.root.populate(depth)
    
    def try_best_position(self, position) -> bool:
        if position.value > self.best_value:
            print("setting best_position")
            self.best_value = position.value
            self.best_position = position
        
        return position.value >= self.best_value

    def best_move(self) -> movement.Move | movement.Castle | None:
        if self.best_position == self.root: return
        position = self.best_position

        #iterate through parents until a top-level pos is found
        while True:
            if position in self.root.children:
                break
            position = position.parent

        #find move that led to position
        for (move, local_pos) in self.root.move_results.items():
            if local_pos == position:
                return move
        
        print("couldn't find position's parent.")
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
        self.parent = parent
    
    def populate(self, depth: int) -> bool:
        good = self.tree.try_best_position(self)
        if not good:
            return False
        elif depth < 1:
            return True
        
        moves = movement.white_legal_moves(self.game) if self.color == board.PieceColor.WHITE else movement.black_legal_moves(self.game)
        random.shuffle(moves)
        for move in moves:
            temp_game = copy.deepcopy(self.game)
            move.perform_on(temp_game)
            new_node = SearchNode(temp_game, self.opponent_color, self.tree, self)
            result = new_node.populate(depth - 1)

            #if it's worse, prune branch
            if not result:
                break
            self.children.append(new_node)
            self.move_results[move] = new_node
        return True

    def __eq__(self, other) -> bool:
        return self.game == other.game
    
    def __ne__(self, other) -> bool:
        return self.game != other.game

