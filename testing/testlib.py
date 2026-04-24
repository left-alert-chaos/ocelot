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

## run_file(fname: str) -> int
Runs one file in testing/ directory. Returns number of tests that failed.
"""
# be nice and allow importing of other modules so I don't have to write this more than I have to
import os
import sys
sys.path.append(os.path.dirname(__file__) + "/..")
sys.path.append(os.path.dirname(__file__) + "/.." + "/src")
import types

tests: list[types.FunctionType] = []
SKIP = ("testlib.py", "__pycache__", "all.sh", "__cache__", "pyc")
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
    return assertion


def register_bool_assertion(assertion: bool, name: str=""):
    tests.append(lambda: bool_assertion(assertion, name))


def run_all(directory: str="."):
    # Move to testing suite directory and make everything importable
    if not os.getcwd().endswith("testing"):
        os.chdir("testing")
    os.chdir(directory)
    print(f"Running test suite at {os.getcwd()}. (Also running files in nested dirs)")

    fail = False
    fails = 0
    queue = []

    # populate beforehand to prevent messy ordering
    for item in os.listdir():
        if item.endswith(SKIP):
            continue
        if os.path.isfile(item):
            print(f"Queueing {item}")
            queue.append(item)
        else:
            print(f"Queueing files in {item}")
            sys.path.append(item)
            for sub in os.listdir(item): queue.append(f"{item}/{sub}")
    
    print(queue)
    for test in queue:
        if test.endswith(SKIP):
            continue

        if not os.path.isfile(test):
            continue
        
        #run tests and check for fail
        print(f"\n\nRunning file {test}")
        result = run_file(test)
        fails += result
        fail = True if result > 0 else False

    print(f"\n\nAll tests complete ({num_of_tests}).")
    if not fail:
        print("All tests in all files passed!")
    else:
        print(f"{fails} fails.")

    #exit with number of failed tests
    sys.exit(fails)


#returns true if all pass
def run_file(fname: str) -> int:
    global num_of_tests

    try:
        # delete file extension and replace / with . for module naming
        tests = __import__(fname.replace(".py", "").replace("/", ".")).testlib.tests
    except Exception as e:
        print(f"Couldn't find testlib module in test file '{fname}'. Skipping.")
        print(f"Error: {e}")
        return 0
    fails = 0

    for test in tests:
        num_of_tests += 1
        if not test():
            fails += 1
        num_of_tests += 1
    print(f"Done running file {fname}. Fails: {fails}")
    return fails


if __name__ == "__main__":
    arg = sys.argv[1] if len(sys.argv) > 1 else "."
    run_all(arg)

