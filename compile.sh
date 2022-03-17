#!/bin/bash

if [ "$1" = "" ]; then
  echo "Usage: $0 <title>"
  exit
fi

cd notes/$1
lualatex *.tex
cd ../..;