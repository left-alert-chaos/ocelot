# Sophisticate (`codon branch`)
This branch holds an alternate version of Sophisticate's source code that is compatible with the Codon compiler. This provides large speedupds across all areas of the engine. The code in this branch is ***not compatible with CPython.***

There are some significant differences between the `codon` and `main` branches, such as the `codon` branch using the type `board.Piece | None` for `square.piece` whereas the `codon` branch using `board.Entity`, used to represent both squares without pieces and squares with pieces.

In general, the codebases are kept similar, but some code will have to be rewritten to transfer from one to the other.

For more information about the Sophisticate project in general, see the `main` branch.
