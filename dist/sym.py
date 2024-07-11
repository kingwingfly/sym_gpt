import sys


def main():
    if len(sys.argv) > 1:
        code = sys.argv[1]
        buffer = {}
        exec(code, buffer)
        expr = buffer["solution"]()
    else:
        print("No command-line arguments provided.")


if __name__ == "__main__":
    main()
