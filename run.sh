#!/bin/bash

cargo build

for ((i=1;i<=25;i++)); do
  if [ -d "$i" ]; then
    cargo run --bin $i
  fi
done