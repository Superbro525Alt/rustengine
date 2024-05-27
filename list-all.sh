find src -type d -name .git -prune -o -type f -name "*.rs" -exec sh -c 'echo "{}" >> output.txt; cat "{}" >> output.txt; echo "" >> output.txt' \;
