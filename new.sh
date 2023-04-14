#!/bin/bash

if [ "$1" = "" ] || [ "$2" = "" ]; then
  echo "Usage: $0 <file> <title>"
  exit
fi

mkdir "notes/$1"

cat << EOF > "notes/$1/$1.tex"
\documentclass[a4paper]{article}

\usepackage{amsmath}
\usepackage{amssymb}
\usepackage{parskip}
\usepackage{fullpage}
\usepackage{hyperref}

\hypersetup{
    colorlinks=true,
    linkcolor=black,
    urlcolor=blue,
    pdftitle={$1},
    pdfpagemode=FullScreen,
}

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

echo "Sample JSON:"
echo ""
cat << EOF
    {
        "title": "$2",
        "file": "$1",
        "tags": [
            "tag1", "tag2"
        ]
    }
EOF