#!/bin/bash

line_separator() {
    # Get the current terminal width
    term_width=$(tput cols)

    # Generate a line of '-' characters with the length of the terminal width
    printf '%*s\n' "$term_width" '' | tr ' ' '-'
}

# Check if tokei and cargo-tree are installed
if ! command -v tokei &> /dev/null
then
    echo "tokei could not be found, installing it..."
    cargo install tokei
fi

if ! command -v cargo-tree &> /dev/null
then
    echo "cargo-tree could not be found, installing it..."
    cargo install cargo-tree
fi

# Count Lines of Code
echo "Counting lines of code..."
tokei_output=$(tokei --output json | jq '.Total.code')

# Count Number of Packages
echo "Counting number of packages..."
packages_count=2

# Count Number of Source Files
echo "Counting number of source files..."
source_files_count=$(find src-tauri -name '*.rs' | wc -l) + $(find src-ui/app src-ui/components src-ui/utils -name '*.js' -o -name '*.jsx' | wc -l)

# Count Number of Dependencies
echo "Counting number of dependencies..."
dependencies_count=$(cd src-tauri&&cargo metadata --no-deps --format-version=1 | jq '.packages[0].dependencies | length') + $(cd ..&&jq '.dependencies | length' package.json) + $(jq '.devDependencies | length' package.json)

# Output the results
line_separator
echo "Lines of Code:"
echo "$tokei_output"
echo
echo "Number of Packages: $packages_count"
line_separator
echo "Number of Source Files: $source_files_count"
line_separator
echo "Number of Dependencies: $dependencies_count"
line_separator
