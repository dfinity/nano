#!/bin/bash

set -ex

export

# Enter temporary directory.
pushd /tmp

brew install coreutils

# install dfx
sh -ci "$(curl -fsSL https://internetcomputer.org/install.sh)"

# Install Bats.
if [ "$(uname -r)" = "19.6.0" ]; then
    brew unlink bats
fi
brew install bats-core

# Install Bats support.
version=0.3.0
curl --location --output bats-support.tar.gz https://github.com/ztombol/bats-support/archive/v$version.tar.gz
mkdir /usr/local/lib/bats-support
tar --directory /usr/local/lib/bats-support --extract --file bats-support.tar.gz --strip-components 1
rm bats-support.tar.gz

# Set environment variables.
BATS_SUPPORT="/usr/local/lib/bats-support"
echo "BATSLIB=${BATS_SUPPORT}" >> "$GITHUB_ENV"
# if this is set, setup_nns.sh will not download all wasm modules for every individual test
echo "DOWNLOAD_DIR=$(mktemp -d)" >> "$GITHUB_ENV"

# Exit temporary directory.
popd
