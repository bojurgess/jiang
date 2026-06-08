#!/usr/bin/env bash
for dir in pkg/bundler pkg/web pkg/node; do
    rm -f "$dir/package.json" "$dir/.gitignore" "$dir/README.md" "$dir/LICENSE"
done
cp LICENSE README.md pkg/