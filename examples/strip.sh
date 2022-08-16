#!/bin/bash

copyright="Copyright (c) MOSEK ApS, Denmark. All rights reserved."

cd $(dirname $0)

for f in *.rs; do
    cat $f \
        | sed -e "s/\/\/\!\([ ]\+\)File.*/\/\/\!\1File : $f/g" \
        | sed -e "s/\/\/\!\([ ]\+\)Copyright.*/\/\/\!\1Copyright : $copyright/g" \
        | sed -e '/TAG:/d' > tmp \
        && cp tmp $f
done
