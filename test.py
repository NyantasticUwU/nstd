import os
import sys

# All modules to test.
MODULES = (
    "nstd_core",
    "nstd_alloc",
    "nstd_audio",
    "nstd_collections",
    "nstd_env",
    "nstd_events",
    "nstd_fs",
    "nstd_gl",
    "nstd_gui",
    "nstd_image",
    "nstd_input",
    "nstd_io",
    "nstd_math",
    "nstd_net",
    "nstd_proc",
    "nstd_str",
    "nstd_sys",
    "nstd_thread",
    "nstd_time"
)

# Iterates through each module, running cargo check for each of them.
def check(target):
    for module in MODULES:
        cmd = f"cargo check --no-default-features --features \"std {module}\""
        if target != None:
            cmd += f" --target={target}"
        print(f"Running {cmd}...")
        os.system(cmd)

# Main entry point of program.
if __name__ == "__main__":
    targets = sys.argv[1:]
    if len(targets) > 0:
        for target in targets:
            check(target)
    else:
        check(None)
