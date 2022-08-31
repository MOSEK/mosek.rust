#!/bin/bash

copyright="Copyright (c) MOSEK ApS, Denmark. All rights reserved."

cd $(dirname $0)

for f in *.rs; do
	cat $f \
		| sed -e 's/\$\$copyright/Copyright (c) MOSEK ApS, Denmark. All rights reserved./g' \
		| sed -e "s/\$\$\{file\}/$f/g"
		| sed -e '/TAG:/d' > tmp
	mv tmp "$f"
done
