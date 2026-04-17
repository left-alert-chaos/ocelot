# Sophisticate
Sophisticate is a chess bot that is decidedly not sophisticated.

It's a simple(ish), dependency-less engine with a modular architecture. All functions are part of modules that depend on each other, and then a UI (graphical or otherwise) simply calls the correct methods to play the engine or for the engine to play itself.

When I say dependency-less, I mean there are no libraries outside of Python's standard library used for the project. No `requirements.txt`. Just an engine from scratch.

# Modules
Inside `src`, each directory holding an `__init__.py` script is a module. The modules and their functions are listed here.

Modules marked unimplemented have not been written yet.

## physical
Physical holds the physical representation of the board, moves, pieces, and colors. It is responsible for finding possible and legal moves and keeping track of which piece is there. As of writing this, it's also the only module close to done.

## evaluation
Evaluation evaluates the board. Shocking, right? It holds the code that reads a board and chooses who is winning and by how much.

## bot
Bot holds the code that ties it all together into one ergonimic (kinda) API. This module provides functions and classes that can actually play a game move-by-move.
