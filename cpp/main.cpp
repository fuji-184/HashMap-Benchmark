#include <iostream>
#include <fstream>
#include <unordered_map>
#include <locale>

uint64_t rdtscp() {
    uint32_t lo, hi, tsc_aux;
    __asm__ __volatile__ (
        "rdtscp" 
        : "=a" (lo), "=d" (hi), "=c" (tsc_aux)
    );
    return ((uint64_t)hi << 32) | lo;
}

int main() {
    int N = 0;
    std::ifstream infile("./input.txt");
    
    if (!(infile >> N)) {
        std::cerr << "Error: Gagal membaca input.txt atau file kosong!\n";
        return 1;
    }

    int64_t get = 0;

    std::unordered_map<int64_t, int64_t> m;

    uint64_t start_insert = rdtscp();
    for (int i = 0; i < N; ++i) {
        m[i] = i;
    }
    uint64_t end_insert = rdtscp();

    uint64_t start_hit = rdtscp();
    for (int i = 0; i < N; ++i) {
        get += m[i];
    }
    uint64_t end_hit = rdtscp();

    int64_t get2 = 0;

    uint64_t start_miss = rdtscp();
    for (int i = N; i < N * 2; ++i) {
        auto it = m.find(i);
        if (it != m.end()) {
            get2 += it->second;
        }
    }
    uint64_t end_miss = rdtscp();

    uint64_t start_update = rdtscp();
    for (int i = 0; i < N; ++i) {
        m[i] = i * 2;
    }
    uint64_t end_update = rdtscp();

    uint64_t start_delete = rdtscp();
    for (int i = 0; i < N; ++i) {
        m.erase(i);
    }
    uint64_t end_delete = rdtscp();

    std::cout.imbue(std::locale("en_US.UTF-8"));
    std::cout << "N: " << N << "\nGet: " << get + get2 << "\n";
    std::cout << "Insert: " << (end_insert - start_insert) << " cycles\n";
    std::cout << "Get Hit: " << (end_hit - start_hit) << " cycles\n";
    std::cout << "Get Miss: " << (end_miss - start_miss) << " cycles\n";
    std::cout << "Update: " << (end_update - start_update) << " cycles\n";
    std::cout << "Delete: " << (end_delete - start_delete) << " cycles\n";

    return 0;
}
