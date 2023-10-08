#!/bin/bash

if [ "$1" = "" ]; then
  echo "Usage: $0 <title>"
  exit
fi

WORKING_DIR=`pwd`
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"

cd $SCRIPT_DIR/notes/$1

tectonic $1.tex -Z search-path=../../packages/

echo ""
echo "Open your file at"
echo $SCRIPT_DIR/notes/$1/$1.pdf

cd $WORKING_DIR
