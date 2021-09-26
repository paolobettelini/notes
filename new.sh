#!/bin/bash

if [ "$1" = "" ] || [ "$2" = "" ]; then
  echo "Usage: $0 <file> <title>"
  exit
fi

mkdir $1

cat << EOF > "$1/$1.tex"
\documentclass{article}
\usepackage[utf8]{inputenc}
\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{parskip}
\usepackage{dsfont}
\usepackage{fullpage}

\title{$2}
\author{Paolo Bettelini}
\date{}

\begin{document}

\maketitle
\tableofcontents
\pagebreak

\section{Abstract}

\pagebreak

\end{document}
EOF