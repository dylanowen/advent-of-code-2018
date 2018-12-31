#!/bin/bash

for ((i=1;i<=25;i++)); do
  if [ -d "$i" ]; then
    (cd ../${i} ; ./build.sh)
  fi
done