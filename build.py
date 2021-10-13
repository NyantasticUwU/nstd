import os
import sys

# All requied modules to be linked.
MODULES = ("audio", "env", "events", "fs", "gui", "image", "io",
           "math", "net", "os", "proc", "str", "thread", "time")

# Main entry point of program.
if __name__ == "__main__":
    if not os.path.exists("lib"):
        os.mkdir("lib")
    for module in MODULES:
        print(f"Building nstd_{module}...")
        os.chdir(f"src/std/{module}")
        os.system("cargo build --release --quiet")
        if sys.platform == "win32":
            if os.path.exists(f"../../../lib/nstd_{module}.lib"):
                os.replace(
                    f"target/release/nstd_{module}.lib", f"../../../lib/nstd_{module}.lib")
            else:
                os.rename(
                    f"target/release/nstd_{module}.lib", f"../../../lib/nstd_{module}.lib")
        os.chdir("../../../")
        print(f"Finished nstd_{module}.")
