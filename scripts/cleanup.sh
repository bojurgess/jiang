#!/usr/bin/env bash
for dir in pkg/bundler pkg/web pkg/nodejs; do
    rm -f "$dir/package.json" "$dir/.gitignore" "$dir/README.md"
done