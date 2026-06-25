#!/bin/bash

set -e

echo "=== 1. COMPILING ==="

echo "[+] Compiling C++ with Clang..."
clang++ -O3 -march=native -std=c++23 cpp/main.cpp -o cpp/main

echo "[+] Compiling Rust..."
cd rust
RUSTFLAGS="-C target-cpu=native" cargo build --release -q
cd ..

echo "[+] Compiling Zig..."
sed -i 's|"src/root.zig"|"src/main.zig"|g' zig/build.zig
cd zig
zig build -Doptimize=ReleaseFast
cd ..

echo "[+] Compiling Go..."
cd golang
go build -o golang .
cd ..

echo -e "\n=== 2. RUNNING BENCHMARKS ===\n"

echo "-----------------------------------"
echo "C++ (std::unordered_map via Clang)"
echo "-----------------------------------"
sleep 1
./cpp/main

echo -e "\n-----------------------------------"
echo "Rust (std::collections::HashMap)"
echo "-----------------------------------"
sleep 1
./rust/target/release/rust

echo -e "\n-----------------------------------"
echo "Zig (std::AutoHashMap)"
echo "-----------------------------------"
sleep 1
./zig/zig-out/bin/zig

echo -e "\n-----------------------------------"
echo "Go (map[int]int)"
echo "-----------------------------------"
sleep 1
./golang/golang

echo -e "\n=== Done ==="
