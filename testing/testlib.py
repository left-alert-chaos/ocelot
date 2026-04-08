"""An ultra-simple testing framework, because I didn't want to learn a real one for this project.

# Functions

## test_function(func: types.FunctionType, expected_result, name: str="") -> bool
Why is everything so self-explanatory?
"""
# be nice and allow importing of other modules so I don't have to write this more than I have to
import os
import sys
sys.path.append(os.path.dirname(__file__) + "/..")
print(os.path.dirname(__file__) + "..")
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

