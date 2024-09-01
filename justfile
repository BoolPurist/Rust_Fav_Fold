# Makes sure if the application builds
check:
    cargo check

# Runs all unit tests
test:
    cargo test

# Runs static analyses to improve code quality
lint:
    cargo clippy

# Checks if code is formatted correctly 
format_check:
    cargo fmt --check --all

# Tries to find errors in spelling. 
# It does not find all possible spelling errors since It focuses on low false positives
spell_check:
   typos 

# Runs all checks relevant to this project.
# Run this command in a CLI environment 
cli: check format_check lint test spell_check  
