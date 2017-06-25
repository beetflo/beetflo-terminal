#!/bin/bash
set -o errexit
set -o pipefail
set -o nounset

BIN_DIR="$( cd "$( dirname "${BASH_SOURCE[0]}" )" && pwd )"
APP_DIR=$(cd $BIN_DIR/../ && pwd)

ICO=$APP_DIR/resources/beetflo.png

sudo cp $ICO /usr/share/pixmaps/
