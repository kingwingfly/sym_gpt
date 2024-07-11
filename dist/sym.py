import sys
from typing import Any
from sympy import pprint


def main():
    if len(sys.argv) > 1:
        code = sys.argv[1]
        buffer: dict[str, Any] = {"print": pprint}
        exec(code, buffer)
        buffer["solution"]()
    else:
        print("No command-line arguments provided.")


if __name__ == "__main__":
    main()
