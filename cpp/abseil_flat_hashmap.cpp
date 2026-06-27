#include <iostream>
#include <fstream>
#include <locale>
#include <cstdint>
#include "absl/container/flat_hash_map.h"

extern "C" {
    int init();
    void start();                                  
    void stop_and_print();
}                                                          

int main() {
    int N = 0;
    std::ifstream infile("../input.txt");

    if (!(infile >> N)) {
        std::cerr << "Error: Gagal membaca input.txt atau file kosong!\n";
        return 1;
    }

    int64_t get = 0;

    absl::flat_hash_map<int64_t, int64_t> m;

    int init_status = init();
    std::cout << "Status Init: " << init_status << std::endl;                                                             
    if (init_status != 0) {
        return 1;
    }

    start();
    for (int i = 0; i < N; ++i) {
        m[i] = i;
    }
    stop_and_print();

    start();
    for (int i = 0; i < N; ++i) {
        get += m[i];
    }
    stop_and_print();

    int64_t get2 = 0;

    start();
    for (int i = N; i < N * 2; ++i) {
        auto it = m.find(i);
        if (it != m.end()) {
            get2 += it->second;
        }
    }
    stop_and_print();

    start();
    for (int i = 0; i < N; ++i) {
        m[i] = i * 2;
    }
    stop_and_print();

    start();
    for (int i = 0; i < N; ++i) {
        m.erase(i);
    }
    stop_and_print();

    std::cout.imbue(std::locale("en_US.UTF-8"));
    std::cout << "N: " << N << "\nGet: " << get + get2 << "\n";

    return 0;
}
