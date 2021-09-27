import os
from os.path import expanduser
import toml

home = expanduser("~")

with open(os.path.join(home, ".config", "cargo-atcoder.toml")) as f:
    d = toml.load(f)
    print(d["project"]['template'])
