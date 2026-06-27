package main

/*
#cgo LDFLAGS: -L../f_count/target/release -lf_count -Wl,-rpath,../f_count/target/release
int init();
void start();
void stop_and_print();
*/
import "C"
import (
	"encoding/binary"
	"fmt"
	"os"
)

func readStrings(path string) []string {
	data, err := os.ReadFile(path)
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: Gagal membuka %s\n", path)
		os.Exit(1)
	}

	n := binary.LittleEndian.Uint64(data[0:8])
	strings := make([]string, 0, n)
	pos := 8

	for i := uint64(0); i < n; i++ {
		length := int(binary.LittleEndian.Uint32(data[pos : pos+4]))
		pos += 4
		s := string(data[pos : pos+length])
		pos += length
		strings = append(strings, s)
	}

	return strings
}

func reverseString(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func main() {
	strings := readStrings("../input.bin")
	n := len(strings)

	initStatus := int(C.init())
	if initStatus == 0 {
		fmt.Println("Status Init: 0")
	}

	get := int64(0)
	get2 := int64(0)
	m := make(map[string]int64)

	fmt.Println("\n=== BENCHMARK: INSERT ===")
	C.start()
	for i, s := range strings {
		m[s] = int64(i)
	}
	C.stop_and_print()

	fmt.Println("\n=== BENCHMARK: GET HIT ===")
	C.start()
	for _, s := range strings {
		if val, ok := m[s]; ok {
			get += val
		}
	}
	C.stop_and_print()

	fmt.Println("\n=== BENCHMARK: GET MISS ===")
	C.start()
	for _, s := range strings {
		miss := reverseString(s)
		if val, ok := m[miss]; ok {
			get2 += val
		}
	}
	C.stop_and_print()

	fmt.Println("\n=== BENCHMARK: RE-INSERT ===")
	C.start()
	for i, s := range strings {
		m[s] = int64(i) * 2
	}
	C.stop_and_print()

	fmt.Println("\n=== BENCHMARK: REMOVE ===")
	C.start()
	for _, s := range strings {
		delete(m, s)
	}
	C.stop_and_print()

	fmt.Printf("\nN: %d\nGet: %d\n", n, get-get2)
}