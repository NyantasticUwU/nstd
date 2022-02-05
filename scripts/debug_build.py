import os

# Main entry point of program.
if __name__ == "__main__":
    os.chdir("../")
    os.system("cargo build --all-features")
