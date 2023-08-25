#!/bin/bash

# Find all .rs files in the src/ directory and its subdirectories
find src/ -type f -name "*.rs" | while read -r file; do
    rustfmt +nightly "$file"
done
