#!/bin/bash

# Exit on error. Append "|| true" if you expect an error.
set -o errexit
# Exit on error inside any functions or subshells.
set -o errtrace
# Do not allow use of undefined vars. Use ${VAR:-} to use an undefined VAR
set -o nounset
# Catch the error in case mysqldump fails (but gzip succeeds) in `mysqldump |gzip`
set -o pipefail
# Turn on traces, useful while debugging but commented out by default
# set -o xtrace

last_release="$(git tag --sort=committerdate | grep -P "v0+\.\d+\.\d+$" | tail -1)"
echo "🐭 Last release: ${last_release}"

# detect breaking changes
if [ -n "$(git log --oneline ${last_release}..HEAD | grep '!:')" ]; then
    echo "🐭 Breaking changes detected since ${last_release}"
    git log --oneline ${last_release}..HEAD | grep '!:'
    # increment the minor version
    minor="${last_release##v0.}"
    minor="${minor%.*}"
    next_minor="$((minor + 1))"
    next_release="v0.${next_minor}.0"
else
    # increment the patch version
    patch="${last_release##*.}"
    next_patch="$((patch + 1))"
    next_release="${last_release/%${patch}/${next_patch}}"
fi
echo "🐭 Next release: ${next_release}"

suffix="alpha"
last_tag="$(git tag --sort=committerdate | tail -1)"
if [[ "${last_tag}" = "${next_release}-${suffix}"* ]]; then
    echo "🐭 Last alpha release: ${last_tag}"
    # increment the alpha version
    # e.g. v0.22.1-alpha.12 -> v0.22.1-alpha.13
    alpha="${last_tag##*-${suffix}.}"
    next_alpha="$((alpha + 1))"
    next_tag="${last_tag/%${alpha}/${next_alpha}}"
else
    # increment the patch and start the alpha version from 0
    # e.g. v0.22.0 -> v0.22.1-alpha.0
    next_tag="${next_release}-${suffix}.0"
fi
# update the crate version
msg="# crate version"
sed -E -i "s/^version = .* ${msg}$/version = \"${next_tag#v}\" ${msg}/" Cargo.toml
echo "NEXT_TAG=${next_tag}" >> $GITHUB_ENV
echo "🐭 Next alpha release: ${next_tag}"
