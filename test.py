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
    "nstd_os",
    "nstd_proc",
    "nstd_thread",
    "nstd_str",
    "nstd_time"
)

# Main entry point of program.
if __name__ == "__main__":
    targets = sys.argv[1:]
    for target in targets:
        for module in MODULES:
            cmd = f"cargo check --no-default-features --features \"std {module}\" --target={target}"
            print(f"Running {cmd}...")
            os.system(cmd)
