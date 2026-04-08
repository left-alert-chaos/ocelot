#!/bin/sh
cd testing
echo "Running all files in /testing directory";
find . -type f -exec python3 {} \;
