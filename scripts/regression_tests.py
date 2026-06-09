# Vertex Regression Test Suite
# Created by Gemini (AI) because i was too lazy to implement is lol (Domio)

import os
import subprocess
import sys

# Colors for output
GREEN = "\033[92m"
RED = "\033[91m"
RESET = "\033[0m"
BOLD = "\033[1m"


def run_command(command):
    # Set VERTEX_RUNTIME_PATH so the compiler can find libvm_runtime.a
    env = os.environ.copy()
    env["VERTEX_RUNTIME_PATH"] = os.path.abspath("vm/libvm_runtime.a")
    process = subprocess.Popen(
        command,
        stdout=subprocess.PIPE,
        stderr=subprocess.PIPE,
        shell=True,
        text=True,
        env=env,
    )
    stdout, stderr = process.communicate()
    return process.returncode, stdout, stderr


def run_test(name, vtx_path, expected_output):
    print(f"Running test {BOLD}{name}{RESET}...", end=" ", flush=True)

    # Use vertexC exec to compile and run
    # Output file name 'test_out' is reused
    cmd = f"./target/debug/vertexC exec {vtx_path} test_out"
    return_code, stdout, stderr = run_command(cmd)

    if return_code != 0:
        print(f"{RED}FAILED (Return Code {return_code}){RESET}")
        print(f"STDOUT: {stdout}")
        print(f"STDERR: {stderr}")
        return False

    # The output of the program is usually the last line(s) before "Program finished"
    # We strip and check if expected_output is in the stdout
    if expected_output in stdout:
        print(f"{GREEN}PASSED{RESET}")
        return True
    else:
        print(f"{RED}FAILED (Output Mismatch){RESET}")
        print(f"Expected to find: '{expected_output}'")
        print(f"Actual output:\n{stdout}")
        return False


def main():
    print(f"{BOLD}Vertex Regression Test Suite{RESET}\n")

    # 1. Build the compiler first
    print("Building Vertex...", end=" ", flush=True)
    ret, _, err = run_command("cargo build --quiet")
    if ret != 0:
        print(f"{RED}Build Failed!{RESET}")
        print(err)
        return
    print(f"{GREEN}Done{RESET}\n")

    # Define tests: (Name, File Path, Expected Output String)
    tests = [
        ("Recursive Fibonacci (fact.vtx)", "testingCode/fact.vtx", "8"),
        ("Function Scope", "testingCode/function_scope.vtx", "5\n12"),
    ]

    # Create a temporary complex test for branching/recursion
    complex_test_path = "testingCode/complex_regression.vtx"
    with open(complex_test_path, "w") as f:
        f.write("""fnc power(base:int, exponent:int):int {
    if (exponent == 0) {
        return 1;
    }
    return base * power(base, exponent - 1);
}

fnc nested_check(a:int):string {
    if (a > 10) {
        if (a < 20) {
            return "Medium";
        }
        return "Large";
    }
    return "Small";
}

var p = power(2, 5);
writeLn!(p);
writeLn!(nested_check(15));
writeLn!(nested_check(5));
writeLn!(nested_check(25));""")

    tests.append(
        ("Complex Branching & Recursion", complex_test_path, "32\nMedium\nSmall\nLarge")
    )

    passed_count = 0
    for name, path, expected in tests:
        if run_test(name, path, expected):
            passed_count += 1

    # Cleanup
    if os.path.exists(complex_test_path):
        os.remove(complex_test_path)
    if os.path.exists("out/test_out"):
        # Not removing binary to avoid errors if it doesn't exist, but usually good practice
        pass

    print(f"\n{BOLD}Results: {passed_count}/{len(tests)} tests passed.{RESET}")
    if passed_count == len(tests):
        print(f"{GREEN}All systems go!{RESET}")
        sys.exit(0)
    else:
        print(f"{RED}Some tests failed.{RESET}")
        sys.exit(1)


if __name__ == "__main__":
    main()
