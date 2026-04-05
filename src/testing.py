"""This file allows for simple testing of other files. It has a few test functions that can be used as unit tests.

It's kinda sloppy, but tests are performed by importing the repective module and running test_function() on one of it's funtions or a wrapping lambda."""
import physical
import types


def test_function(func: types.FunctionType, expected_result, name: str="") -> bool:
    fail = False
    err = None
    fail_reason = ""
    try:
        res = func()
        if res != expected_result:
            fail = True
            fail_reason = f"Result '{res}' doesn't match expected result '{expected_result}'"
    except Exception as e:
        fail = True
        fail_reason = "Uncaught exception"
        err = e
    
    if fail:
        print(f"Function test '{name}' failed due to {fail_reason}. Err: {err}")
    else:
        print(f"Function test '{name}' succeeded!")
    return not fail


game = physical.board.Board()
move = physical.board.Move("b", 5, "c", 4)
test_function(lambda: game.perform(move), None, "Illegal move")

