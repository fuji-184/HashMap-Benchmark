#include <iostream>
#include <fstream>
#include <vector>
#include <string>
#include <cstdint>
#include "absl/container/flat_hash_map.h"

extern "C" {
    void init();
    void start();
    void stop();
    void print();
    void reset();
}

enum class InputType { String, Integer };

InputType detect_type(const char* path) {
    std::ifstream file(path, std::ios::binary);
    if (!file) { std::cerr << "Error: cannot open " << path << "\n"; exit(1); }

    uint64_t n = 0;
    file.read(reinterpret_cast<char*>(&n), 8);

    uint8_t type_byte = 0;
    file.read(reinterpret_cast<char*>(&type_byte), 1);

    return (type_byte == 1) ? InputType::Integer : InputType::String;
}

std::vector<std::string> read_strings(const char* path) {
    std::ifstream file(path, std::ios::binary);
    uint64_t n = 0;
    file.read(reinterpret_cast<char*>(&n), 8);
    file.seekg(1, std::ios::cur);

    std::vector<std::string> result;
    result.reserve(n);
    for (uint64_t i = 0; i < n; ++i) {
        uint32_t len = 0;
        file.read(reinterpret_cast<char*>(&len), 4);
        std::string s(len, '\0');
        file.read(s.data(), len);
        result.push_back(std::move(s));
    }
    return result;
}

std::vector<uint64_t> read_integers(const char* path) {
    std::ifstream file(path, std::ios::binary);
    uint64_t n = 0;
    file.read(reinterpret_cast<char*>(&n), 8);
    file.seekg(1, std::ios::cur);

    std::vector<uint64_t> result(n);
    file.read(reinterpret_cast<char*>(result.data()), n * 8);
    return result;
}

template<typename K>
void run_benchmarks(const std::vector<K>& keys) {
    size_t n = keys.size();
    int64_t get = 0, get2 = 0;

    absl::flat_hash_map<K, int64_t> m;
    m.reserve(keys.size());

    std::cout << "\n=== BENCHMARK: INSERT ===" << std::endl;
    for (size_t i = 0; i < n; ++i) {
        start();
        m[keys[i]] = static_cast<int64_t>(i);
        stop();
    }
    print();
    reset();

    std::cout << "\n=== BENCHMARK: GET HIT ===" << std::endl;
    for (const auto& k : keys) {
        start();
        auto it = m.find(k);
        stop();
        if (it != m.end()) get += it->second;
    }
    print();
    reset();

    std::cout << "\n=== BENCHMARK: GET MISS ===" << std::endl;
    for (const auto& k : keys) {
        K miss;
        if constexpr (std::is_same_v<K, std::string>)
            miss = std::string(k.rbegin(), k.rend());
        else
            miss = k + 1;
        start();
        auto it = m.find(miss);
        stop();
        if (it != m.end()) get2 += it->second;
    }
    print();
    reset();

    std::cout << "\n=== BENCHMARK: RE-INSERT ===" << std::endl;
    for (size_t i = 0; i < n; ++i) {
        auto i2 = static_cast<int64_t>(i * 2);
        start();
        m[keys[i]] = i2;
        stop();
    }
    print();
    reset();

    std::cout << "\n=== BENCHMARK: REMOVE ===" << std::endl;
    for (const auto& k : keys) {
        start();
        m.erase(k);
        stop();
    }
    print();
    reset();

    std::cout << "\nN: " << n << "\nGet: " << (get - get2) << "\n";
}

int main() {
    const char* path = "../input.bin";
    init();

    switch (detect_type(path)) {
        case InputType::Integer:
            std::cout << "[i] Detected: integer\n";
            run_benchmarks(read_integers(path));
            break;
        case InputType::String:
            std::cout << "[i] Detected: string\n";
            run_benchmarks(read_strings(path));
            break;
    }

    return 0;
}