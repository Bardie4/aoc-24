#!/bin/bash

# Create and populate module files for each day
for i in $(seq -w 1 25); do
    filename="src/day${i}.rs"
    echo "Creating $filename"
    cat <<EOL > $filename
pub fn solve() {
    // Implementation for day $i will go here
}
EOL
done

echo "All module files created and populated."