#!/usr/bin/python3.6

from pathlib import Path
import os
import shutil
import argparse
import subprocess

"""\
new -> make new folder
open -> open folder with peco
"""

ROOT_DIRECTORY = Path(os.path.dirname(os.path.abspath(__file__))).parent
PROBLEM_DIRECTORY = os.path.join(ROOT_DIRECTORY, "problems")
ETC_DIRECTORY = os.path.join(ROOT_DIRECTORY, "etc")


def make_toolchain(path):
    with open(os.path.join(path, "rust-toolchain"), "w") as f:
        f.write("1.42.0")


def ln_makefile(path):
    target = os.path.join(path, "Makefile.toml")
    source = os.path.join(ETC_DIRECTORY, "Makefile.toml")
    subprocess.run(f"ln -sfv {source} {target}", shell=True)


def new(args):
    contest_dirs = [d for d in os.listdir(PROBLEM_DIRECTORY)]
    contest_path = os.path.join(PROBLEM_DIRECTORY, args.contest_id)

    os.chdir(PROBLEM_DIRECTORY)
    if args.contest_id not in contest_dirs:
        subprocess.run(f"cargo atcoder new {args.contest_id}", shell=True, check=True)
        make_toolchain(contest_path)
        ln_makefile(contest_path)

        subprocess.run(["git", "add", "."])
        subprocess.run(["git", "commit", "-m", f'"first commit in {args.contest_id}"'])

    subprocess.run(f"code {args.contest_id}", shell=True, check=True)


def parse_args():
    parser = argparse.ArgumentParser()
    subparsers = parser.add_subparsers()

    # new
    new_parser = subparsers.add_parser("new")
    new_parser.add_argument("contest_id")
    new_parser.set_defaults(handler=new)

    # args
    args = parser.parse_args()

    if hasattr(args, "handler"):
        args.handler(args)
    else:
        parser.print_help()


def main():
    parse_args()


if __name__ == "__main__":
    main()
