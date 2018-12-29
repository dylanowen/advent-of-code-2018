#!/bin/bash

cargo build --release

for ((i=1;i<=25;i++)); do
  if [ -d "$i" ]; then
    cargo run --release --bin $i
  fi
done