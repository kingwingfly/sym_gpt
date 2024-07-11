import sys
from sympy import pprint


def main():
    if len(sys.argv) > 1:
        code = sys.argv[1]
        buffer = {}
        exec(code, buffer)
        expr = buffer["formula"]()
        pprint(expr)
    else:
        print("No command-line arguments provided.")


if __name__ == "__main__":
    main()
