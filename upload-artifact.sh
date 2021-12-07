#!/bin/bash
set -u
echo "$GITHUB_TOKEN" >.githubtoken
unset GITHUB_TOKEN
gh auth login --with-token <.githubtoken
rm .githubtoken
gh release upload "$VERSION" "$1" --clobber
