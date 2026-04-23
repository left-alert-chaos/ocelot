"""An ultra-simple testing framework, because I didn't want to learn a real one for this project.
When run on its own, it calls run_all() and reports on each file.
If a command-line argument is provided, it is passed as an argument to run_all().

# Exit codes
An exit code of 0 means that all tests passed.
Any other code is the number of tests that failed.

# Functions

## test_function(func: types.FunctionType, expected_result, name: str="") -> bool
Why is everything so self-explanatory?

## register_function_test(func: types.FunctionType, expected_result, name: str="")
Call this to register a test. Registered tests are run when testlib finds a test file in the testing/ directory.

## bool_assertion(assertion: bool, name: str="") -> bool
Checks whether assertion is True. Returns assertion.

## register_bool_assertion(assertion: bool, name: str="")
Call this to register an assertion. Registered assertions and tests are run when testlib finds a test file in the testing/ directory.

## run_all(directory=".")
Runs all python files in testing/. directory.
If the directory argument is specified, that directory is moved to after moving to testing. This can be used to specify which testing suite to run.
Exits with a code defined above.

## run_file(fname: str) -> bool
Runs one file in testing/ directory. Returns True if all tests pass in file.
"""
# be nice and allow importing of other modules so I don't have to write this more than I have to
import os
import sys
sys.path.append(os.path.dirname(__file__) + "/..")
import types

tests: list[types.FunctionType] = []
SKIP = ("testlib.py", "__pycache__", "all.sh", "__cache__")
fails = 0
num_of_tests = 0


def test_function(func: types.FunctionType | types.MethodType, expected_result, name: str="") -> bool:
    global fails
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
        fails += 1
    else:
        print(f"Function test '{name}' succeeded!")
    return not fail


def register_function_test(func: types.FunctionType, expected_result, name: str=""):
    tests.append(lambda: test_function(func, expected_result, name))


def bool_assertion(assertion: bool, name: str="") -> bool:
    global fails
    if assertion:
        print(f"Bool assertion '{name}' succeeded (True)!")
    else:
        print(f"Bool assertion '{name}' failed (False).")
        fails += 1
    return assertion


def register_bool_assertion(assertion: bool, name: str=""):
    tests.append(lambda: bool_assertion(assertion, name))


def run_all(directory: str="."):
    global fails
    # Move to testing suite directory
    if not os.getcwd().endswith("testing"):
        os.chdir("testing")
    os.chdir(directory)
    loc = os.getcwd()
    print(f"Running test suite at {loc}. (Also running files in nested dirs)")

    #make local files importable
    sys.path.append(loc)

    fail = False
    queue = os.listdir()
    
    while True:
        # queue management
        if len(queue) == 0:
            break
        test = queue[0]
        queue.pop(0)

        #skips
        if test.endswith(SKIP):
            continue

        # add nested files
        if os.path.isdir(test):
            sys.path.append(test)
            for nested in os.listdir(test):
                queue.append(nested)
            continue
        
        #run tests and check for fail
        fail = True if not run_file(test) else False

    print(f"\n\nAll tests complete ({num_of_tests}).")
    if not fail:
        print("All tests in all files passed!")
    else:
        print(f"{fails} fails.")

    #exit with number of failed tests
    sys.exit(fails)


#returns true if all pass
def run_file(fname: str) -> bool:
    global num_of_tests

    try:
        tests = __import__(fname.replace(".py", "")).testlib.tests
    except Exception as e:
        print(f"Couldn't find testlib module in test file '{fname}'. Skipping.")
        print(f"Error: {e}")
        return True
    fail = False

    for test in tests:
        num_of_tests += 1
        if not test():
            fail = True
    return not fail


if __name__ == "__main__":
    arg = sys.argv[1] if len(sys.argv) > 1 else "."
    run_all(arg)

