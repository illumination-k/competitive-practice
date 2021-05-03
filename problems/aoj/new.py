import os
import sys

def main():
    new_problem = sys.argv[1:][0]

    with open("template.rs") as f:
        lines = f.readlines()

    if not new_problem.endswith(".rs"):
        new_problem += ".rs"

    path = os.path.join("src/bin", new_problem)
    with open(path, "w") as w:
        w.write("".join(lines))

if __name__ == "__main__":
    main()