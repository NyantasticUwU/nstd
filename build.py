import os
import sys

# All requied modules to be linked.
MODULES = ("alloc", "audio", "collections", "env", "events", "fs", "gl",
           "gui", "image", "io", "math", "net", "os", "proc", "str", "thread", "time")

# Main entry point of program.
if __name__ == "__main__":
    args = None
    argc = len(sys.argv)
    if argc > 1:
        args = ""
        for i in range(1, argc):
            args += f"{sys.argv[i]} "
    if not os.path.exists("lib"):
        os.mkdir("lib")
    for module in MODULES:
        print(f"Building nstd_{module}...")
        os.chdir(f"src/std/{module}")
        if args == None:
            os.system("cargo build --release --quiet")
            if sys.platform.startswith("win32"):
                SRC = f"target/release/nstd_{module}.lib"
                DEST = f"../../../lib/nstd_{module}.lib"
                if os.path.exists(DEST):
                    os.replace(SRC, DEST)
                else:
                    os.rename(SRC, DEST)
            elif sys.platform.startswith("linux"):
                SRC = f"target/release/libnstd_{module}.a"
                DEST = f"../../../lib/libnstd_{module}.a"
                if os.path.exists(DEST):
                    os.replace(SRC, DEST)
                else:
                    os.rename(SRC, DEST)
        else:
            os.system(f"cargo {args}")
        os.chdir("../../../")
        print(f"Finished nstd_{module}.")
