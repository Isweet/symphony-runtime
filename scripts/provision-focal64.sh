#!/bin/bash

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)

$DIR/../extern/MOTION/scripts/provision-focal64.sh
