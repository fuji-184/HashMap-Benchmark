package main

/*
#cgo LDFLAGS: -L../f_count/target/release -lf_count -Wl,-rpath,../f_count/target/release
void init();
void start();
void stop();
void print();
void reset();
*/
import "C"
import (
	"encoding/binary"
	"fmt"
	"os"
)

type InputType int

const (
	TypeString  InputType = 0
	TypeInteger InputType = 1
)

func detectType(data []byte) InputType {
	if data[8] == 1 {
		return TypeInteger
	}
	return TypeString
}

func readStrings(data []byte) []string {
	n := binary.LittleEndian.Uint64(data[0:8])
	result := make([]string, 0, n)
	pos := 9
	for i := uint64(0); i < n; i++ {
		length := int(binary.LittleEndian.Uint32(data[pos : pos+4]))
		pos += 4
		s := string(data[pos : pos+length])
		pos += length
		result = append(result, s)
	}
	return result
}

func readIntegers(data []byte) []uint64 {
	n := binary.LittleEndian.Uint64(data[0:8])
	result := make([]uint64, 0, n)
	pos := 9
	for i := uint64(0); i < n; i++ {
		val := binary.LittleEndian.Uint64(data[pos : pos+8])
		pos += 8
		result = append(result, val)
	}
	return result
}

func reverseString(s string) string {
	runes := []rune(s)
	for i, j := 0, len(runes)-1; i < j; i, j = i+1, j-1 {
		runes[i], runes[j] = runes[j], runes[i]
	}
	return string(runes)
}

func runBenchmarksString(keys []string) {
	n := len(keys)
	get := int64(0)
	get2 := int64(0)
	m := make(map[string]int64, n)

	fmt.Println("\n=== BENCHMARK: INSERT ===")
	for i, k := range keys {
		C.start()
		m[k] = int64(i)
		C.stop()
	}
	C.print()
	C.reset()

	fmt.Println("\n=== BENCHMARK: GET HIT ===")
	for _, k := range keys {
		C.start()
		val, ok := m[k]
		C.stop()
		if ok {
			get += val
		}
	}
	C.print()
	C.reset()

	fmt.Println("\n=== BENCHMARK: GET MISS ===")
	for _, k := range keys {
		miss := reverseString(k)
		C.start()
		val, ok := m[miss]
		C.stop()
		if ok {
			get2 += val
		}
	}
	C.print()
	C.reset()

	fmt.Println("\n=== BENCHMARK: RE-INSERT ===")
	for i, k := range keys {
		v := int64(i) * 2
		C.start()
		m[k] = v
		C.stop()
	}
	C.print()
	C.reset()

	fmt.Println("\n=== BENCHMARK: REMOVE ===")
	for _, k := range keys {
		C.start()
		delete(m, k)
		C.stop()
	}
	C.print()
	C.reset()

	fmt.Printf("\nN: %d\nGet: %d\n", n, get-get2)
}

func runBenchmarksInteger(keys []uint64) {
	n := len(keys)
	get := int64(0)
	get2 := int64(0)
	m := make(map[uint64]int64, n)

	fmt.Println("\n=== BENCHMARK: INSERT ===")
	for i, k := range keys {
		C.start()
		m[k] = int64(i)
		C.stop()
	}
	C.print()
	C.reset()

	fmt.Println("\n=== BENCHMARK: GET HIT ===")
	for _, k := range keys {
		C.start()
		val, ok := m[k]
		C.stop()
		if ok {
			get += val
		}
	}
	C.print()
	C.reset()

	fmt.Println("\n=== BENCHMARK: GET MISS ===")
	for _, k := range keys {
		miss := k + 1
		C.start()
		val, ok := m[miss]
		C.stop()
		if ok {
			get2 += val
		}
	}
	C.print()
	C.reset()

	fmt.Println("\n=== BENCHMARK: RE-INSERT ===")
	for i, k := range keys {
		v := int64(i) * 2
		C.start()
		m[k] = v
		C.stop()
	}
	C.print()
	C.reset()

	fmt.Println("\n=== BENCHMARK: REMOVE ===")
	for _, k := range keys {
		C.start()
		delete(m, k)
		C.stop()
	}
	C.print()
	C.reset()

	fmt.Printf("\nN: %d\nGet: %d\n", n, get-get2)
}

func main() {
	data, err := os.ReadFile("../input.bin")
	if err != nil {
		fmt.Fprintf(os.Stderr, "Error: cannot open ../input.bin\n")
		os.Exit(1)
	}

	C.init()

	switch detectType(data) {
	case TypeInteger:
		fmt.Println("[i] Detected: integer")
		runBenchmarksInteger(readIntegers(data))
	case TypeString:
		fmt.Println("[i] Detected: string")
		runBenchmarksString(readStrings(data))
	}
}