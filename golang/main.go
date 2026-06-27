package main

/*
#cgo LDFLAGS: -L../f_count/target/release -lf_count -Wl,-rpath,../f_count/target/release
int init();
void start();
void stop_and_print();
*/
import "C"

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func main() {
	N := 0
	data, err := os.ReadFile("../input.txt")
	if err == nil {
		trimmed := strings.TrimSpace(string(data))
		if val, err := strconv.Atoi(trimmed); err == nil {
			N = val
		}
	}

	// Inisialisasi perf counter melalui CGO
	initStatus := int(C.init())
	fmt.Printf("Status Init: %d\n", initStatus)
	if initStatus != 0 {
		os.Exit(1)
	}

	get := 0
	m := make(map[int]int)

	fmt.Println("\n=== BENCHMARK: INSERT ===")
	C.start()
	for i := 0; i < N; i++ {
		m[i] = i
	}
	C.stop_and_print()

	fmt.Println("\n=== BENCHMARK: GET HIT ===")
	C.start()
	for i := 0; i < N; i++ {
		get += m[i]
	}
	C.stop_and_print()

	get2 := 0

	fmt.Println("\n=== BENCHMARK: GET MISS ===")
	C.start()
	for i := N; i < N*2; i++ {
		get2 += m[i]
	}
	C.stop_and_print()

	fmt.Println("\n=== BENCHMARK: UPDATE ===")
	C.start()
	for i := 0; i < N; i++ {
		m[i] = i * 2
	}
	C.stop_and_print()

	fmt.Println("\n=== BENCHMARK: DELETE ===")
	C.start()
	for i := 0; i < N; i++ {
		delete(m, i)
	}
	C.stop_and_print()

	fmt.Printf("\nN: %d\nGet: %d\n", N, get+get2)
}
