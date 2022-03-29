#!/bin/bash

DIR=$(CDPATH= cd -- "$(dirname -- "$0")" && pwd -P)

../extern/MOTION/scripts/provision-focal64.sh
