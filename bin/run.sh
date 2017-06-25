#!/bin/bash
set -o errexit
set -o pipefail
set -o nounset

BIN_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
APP_DIR=$(cd $BIN_DIR/../ && pwd)

cd $APP_DIR
cargo build --debug



