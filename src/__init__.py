"""The main, top-level module of Sophisticate. If you're importing it, you're probably writing a test.

# Modules

## physical
Holds anything having to do with the board or positioning. Moves, Pieces, Piece colors, etc

## inference
What one could call the actual engine.
Not implemented yet."""

# allow importing
import os
import sys
sys.path.append(os.path.dirname(__file__))

import physical
import evaluation

