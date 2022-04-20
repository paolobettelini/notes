#!/bin/bash

if [ "$1" = "" ]; then
  echo "Usage: $0 <title> [--bibtex]"
  echo ""
  echo "--bibtex   Compile new bibtex references using biber"
  exit
fi

WORKING_DIR=`pwd`
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"

cd $SCRIPT_DIR/notes/$1

lualatex *.tex

if [ "$1" = "--bibtex" ]; then
    biber $1
    lualatex *.tex
    lualatex *.tex
fi

cd $WORKING_DIR