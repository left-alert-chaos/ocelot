"""A module containing clean APIs that abstract away all other messiness. Just a bot that plays chess.

# Modules

## uci
A module providing an implementation of the UCI standard.

# Classes

## Sophisticate
The bot.

## GameException(Exception)
An exception for use when no legal moves could be found or some other reason prevents a move from being found or played.

## WinException(GameException)
An exception raised if the bot wins.

## LossException(GameException)
An exception raised if the bot loses.

## StalemateException(GameException)
An exception raised if no legal moves could be found but there isn't check."""

import os
import sys
dirname = os.path.dirname(__file__)
if dirname not in sys.path:
    sys.path.append(dirname)
import uci
from robot import Sophisticate, GameException, WinException, LossException, StalemateException


