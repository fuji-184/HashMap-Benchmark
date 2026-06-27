#include <iostream>
#include <fstream>
#include <unordered_map>
#include <vector>
#include <string>
#include <algorithm>
#include <cstdint>

extern "C" {
    int init();
    void start();
    void stop_and_print();
}

std::vector<std::string> read_strings(const char* path) {
    std::ifstream file(path, std::ios::binary);
    if (!file) {
        std::cerr << "Error: Gagal membuka " << path << "\n";
        exit(1);
    }

    uint64_t n = 0;
    file.read(reinterpret_cast<char*>(&n), 8);

    std::vector<std::string> strings;
    strings.reserve(n);

    for (uint64_t i = 0; i < n; ++i) {
        uint32_t len = 0;
        file.read(reinterpret_cast<char*>(&len), 4);
        std::string s(len, '\0');
        file.read(&s[0], len);
        strings.push_back(std::move(s));
    }

    return strings;
}

int main() {
    auto strings = read_strings("../input.bin");
    size_t n = strings.size();

    int64_t get = 0;
    int64_t get2 = 0;
    std::unordered_map<std::string, int64_t> m;

    int init_status = init();
    if (init_status == 0) {
        std::cout << "Status Init: 0\n";
    }

    std::cout << "\n=== BENCHMARK: INSERT ===" << std::endl;
    start();
    for (size_t i = 0; i < n; ++i) {
        m[strings[i]] = static_cast<int64_t>(i);
    }
    stop_and_print();

    std::cout << "\n=== BENCHMARK: GET HIT ===" << std::endl;
    start();
    for (const auto& s : strings) {
        auto it = m.find(s);
        if (it != m.end()) {
            get += it->second;
        }
    }
    stop_and_print();

    std::cout << "\n=== BENCHMARK: GET MISS ===" << std::endl;
    start();
    for (const auto& s : strings) {
        std::string miss(s.rbegin(), s.rend());
        auto it = m.find(miss);
        if (it != m.end()) {
            get2 += it->second;
        }
    }
    stop_and_print();

    std::cout << "\n=== BENCHMARK: RE-INSERT ===" << std::endl;
    start();
    for (size_t i = 0; i < n; ++i) {
        m[strings[i]] = static_cast<int64_t>(i) * 2;
    }
    stop_and_print();

    std::cout << "\n=== BENCHMARK: REMOVE ===" << std::endl;
    start();
    for (const auto& s : strings) {
        m.erase(s);
    }
    stop_and_print();

    std::cout << "\nN: " << n << "\nGet: " << (get - get2) << "\n";
    return 0;
}