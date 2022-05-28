#!/bin/bash

if [ "$1" = "" ]; then
  echo "Usage: $0 <title> [--bibtex] [--lilypond]"
  echo ""
  echo "--bibtex   Compile new bibtex references using biber"
  exit
fi

WORKING_DIR=`pwd`
SCRIPT_DIR="$(dirname "$(readlink -f "$0")")"

cd $SCRIPT_DIR/notes/$1

if [ "$2" = "--bibtex" ]; then
    lualatex *.tex
    biber $1
    lualatex *.tex
    lualatex *.tex
elif [ "$2" = "--lilypond" ]; then
    lilypond-book --pdf *.tex --out=target
    lilypond-book --pdf *.tex --out=target
    cd target
    lualatex *.tex
    mv *.pdf ../
    cd ..
    find . -type f -name "tmp*.pdf" -delete
    find . -type f -name "tmp*.out" -delete
else
    lualatex *.tex
fi

cd $WORKING_DIR
