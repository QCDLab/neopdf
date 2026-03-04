#!/bin/bash

# Adapted from https://github.com/NNPDF/pineappl/blob/master/maintainer/make-release.sh

set -euo pipefail

crates=(
    neopdf
    neopdf_capi
    neopdf_cli
    neopdf_pyapi
)

main=master
this_branch=$(git rev-parse --abbrev-ref HEAD)

cd ..

if [[ $# != 1 ]]; then
    echo "No version number given."
    exit 1
fi

version=$1

prerelease=$(echo ${version} | perl -pe 's/^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$/\4/')

if [[ $(echo ${version} | grep -oP '^(0|[1-9]\d*)\.(0|[1-9]\d*)\.(0|[1-9]\d*)(?:-((?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*)(?:\.(?:0|[1-9]\d*|\d*[a-zA-Z-][0-9a-zA-Z-]*))*))?(?:\+([0-9a-zA-Z-]+(?:\.[0-9a-zA-Z-]+)*))?$') != ${version} ]]; then
    echo "Version string incorrect."
    exit 1
fi

if [[ $(git tag -l v${version}) ]]; then
    echo "Version already exists."
    exit 1
fi

# in branches that are not master we only allow prereleases
if [[ ${this_branch} != ${main} ]] && [[ ${prerelease} == "" ]]; then
    echo "Ordinary releases are only allowed in the '${main}' branch."
    echo "If you really want to make a release from '${this_branch}', consider making a prerelease."
    exit 1
fi

echo ">>> Updating version strings ..."

# we don't want to create a changelog entry for prereleases, which are solely
# for internal testing purposes
if [[ ${prerelease} == "" ]]; then
    sed -i \
        -e "s:\(## \[Unreleased\]\):\1\n\n## [${version}] - $(date +%d/%m/%Y):" \
        -e "s:\[Unreleased\]\(\: https\://github.com/NNPDF/neopdf/compare/v\)\(.*\)...HEAD:[Unreleased]\1${version}...HEAD\n[${version}]\1\2...v${version}:" \
        CHANGELOG.md
    git add CHANGELOG.md
fi

# modify the versions in the main workspace
sed -i \
    -e "/^\[workspace\.package\]/,/^\[/ s:^version = \".*\":version = \"${version}\":" \
    -e "s:^neopdf = { path = \"[^\"]*\", version = \"[^\"]*\" }:neopdf = { path = \"./neopdf\", version = \"${version}\" }:" \
    Cargo.toml
git add Cargo.toml

# update Tauri GUI version
sed -i "s/\"version\": *\"[^\"]*\"/\"version\": \"${version}\"/" neopdf_gui/tauri.conf.json
git add neopdf_gui/tauri.conf.json

# update Pixi version
sed -i "/^\[workspace\]/,/^\[/ s:^version = \".*\":version = \"${version}\":" pixi.toml
git add pixi.toml

echo ">>> Updating Cargo.lock ..."

# update explicit version for `neopdf_tmdlib` in `neopdf_cli`
sed -i \
    -e "s:neopdf_tmdlib = { path = \"[^\"]*\", version = \"[^\"]*\", optional = true }:neopdf_tmdlib = { path = \"../neopdf_tmdlib\", version = \"${version}\", optional = true }:" \
    neopdf_cli/Cargo.toml
git add neopdf_cli/Cargo.toml

for crate in "${crates[@]}"; do
    # convert packages in the lockfile that correspond to files in this
    # repository to PKGIDs - important because we may depend on crate with
    # different version multiple times
    cargo pkgid path+file://$(cd "${crate}" && pwd)
done | xargs printf " -p %s" | xargs cargo update
git add Cargo.lock

echo ">>> Commiting and pushing changes ..."

git commit -m "Release v${version}"
git tag -a v${version} -m v${version}
git push --follow-tags
