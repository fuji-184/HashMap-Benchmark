#!/bin/bash

clear

set -e

echo "=== 1. COMPILING ==="

echo "[+] Compiling C++ with Clang..."
cd cpp
clang++ -O3 -march=native -std=c++23 main.cpp -o main -L../f_count/target/release -lf_count -Wl,-rpath,../f_count/target/release

echo "[+] Compiling C++ Abseil Swiss Table with Clang..."
#clang++ -O3 -march=native -std=c++23 abseil_flat_hashmap.cpp -o abseil_flat_hashmap -labsl_raw_hash_set -labsl_hash -labsl_city -L../f_count/target/release -lf_count -Wl,-rpath,../f_count/target/release
clang++ -O3 -march=native -std=c++23 abseil_flat_hashmap.cpp -o abseil_flat_hashmap \
  $(pkg-config --libs absl_flat_hash_map) \
  -L../f_count/target/release -lf_count -Wl,-rpath,../f_count/target/release

echo "[+] Compiling Rust..."
cd ../rust
RUSTFLAGS="-C target-cpu=native" cargo build --release -q
cd ..

echo "[+] Compiling Rust Rustc_Hash library..."
cd rustc_hash
RUSTFLAGS="-C target-cpu=native" cargo build --release
cd ..

echo "[+] Compiling Rust Ahash library..."
cd rust_ahash
RUSTFLAGS="-C target-cpu=native" cargo build --release
cd ..

echo "[+] Compiling Rust Hashbrown library..."
cd rust_hashbrown
RUSTFLAGS="-C target-cpu=native" cargo build --release
cd ..

#echo "[+] Compiling Rust F_Map..."
#cd f_map
#RUSTFLAGS="-C target-cpu=native" cargo build --release
#cd ..

echo "[+] Compiling Zig..."
sed -i 's|"src/root.zig"|"src/main.zig"|g' zig/build.zig
cd zig
#zig build -Doptimize=ReleaseFast
zig build-exe src/main.zig -O ReleaseFast -lc -L ../f_count/target/release -lf_count -rpath ../f_count/target/release
cd ..

echo "[+] Compiling Go..."
cd golang
CGO_ENABLED=1 go build -o golang main.go
cd ..

echo -e "\n=== 2. RUNNING BENCHMARKS ===\n"

echo "-----------------------------------"
echo "C++ (std::unordered_map via Clang)"
echo "-----------------------------------"
cd cpp
sleep 1
./main

echo "-----------------------------------"
echo "C++ (absl::flat_hash_map via Clang)"
echo "-----------------------------------"
sleep 1
./abseil_flat_hashmap

cd ../rust
echo -e "\n-----------------------------------"
echo "Rust (std::collections::HashMap)"
echo "-----------------------------------"
sleep 1
./target/release/rust

echo -e "\n-----------------------------------"
echo "Rust (rustc_hash::FxHashMap)"
echo "-----------------------------------"
cd ../rustc_hash
sleep 1
./target/release/rust

echo -e "\n-----------------------------------"
echo "Rust (ahash::AHashMap)"
echo "-----------------------------------"
cd ../rust_ahash
sleep 1
./target/release/rust

echo -e "\n-----------------------------------"
echo "Rust (hashbrown::HashMap;)"
echo "-----------------------------------"
cd ../rust_hashbrown
sleep 1
./target/release/rust

#echo -e "\n-----------------------------------"
#echo "Rust (F_Map)"
#echo "-----------------------------------"
#cd ../f_map
#sleep 1
#./target/release/rust

echo -e "\n-----------------------------------"
echo "Zig (std::AutoHashMap)"
echo "-----------------------------------"
cd ../zig
sleep 1
#./zig/zig-out/bin/zig
./main

echo -e "\n-----------------------------------"
echo "Go (map[int]int)"
echo "-----------------------------------"
cd ../golang
sleep 1
./golang

cd ..
echo -e "\n=== Done ==="
