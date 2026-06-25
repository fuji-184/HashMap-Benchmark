package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func rdtscp() uint64

func formatCycles(n uint64) string {
	in := strconv.FormatUint(n, 10)
	numOfDigits := len(in)
	if numOfDigits <= 3 {
		return in
	}
	out := make([]byte, len(in)+(len(in)-1)/3)
	inI := numOfDigits - 1
	outI := len(out) - 1
	for inI >= 0 {
		out[outI] = in[inI]
		outI--
		inI--
		if inI >= 0 && (numOfDigits-1-inI)%3 == 0 {
			out[outI] = '.'
			outI--
		}
	}
	return string(out)
}

func main() {
	N := 0
	data, err := os.ReadFile("./input.txt")
	if err == nil {
		trimmed := strings.TrimSpace(string(data))
		if val, err := strconv.Atoi(trimmed); err == nil {
			N = val
		}
	}

        get := 0
	m := make(map[int]int)

	startInsert := rdtscp()
	for i := 0; i < N; i++ {
		m[i] = i
	}
	endInsert := rdtscp()

	startHit := rdtscp()
	for i := 0; i < N; i++ {
		get += m[i]
	}
	endHit := rdtscp()

        get2 := 0

	startMiss := rdtscp()
	for i := N; i < N*2; i++ {
		get2 += m[i]
	}
	endMiss := rdtscp()

	startUpdate := rdtscp()
	for i := 0; i < N; i++ {
		m[i] = i * 2
	}
	endUpdate := rdtscp()

	startDelete := rdtscp()
	for i := 0; i < N; i++ {
		delete(m, i)
	}
	endDelete := rdtscp()

        fmt.Printf("N: %d\nGet: %d\n", N, get + get2)
	fmt.Printf("Insert: %s cycles\n", formatCycles(endInsert-startInsert))
	fmt.Printf("Get Hit: %s cycles\n", formatCycles(endHit-startHit))
	fmt.Printf("Get Miss: %s cycles\n", formatCycles(endMiss-startMiss))
	fmt.Printf("Update: %s cycles\n", formatCycles(endUpdate-startUpdate))
	fmt.Printf("Delete: %s cycles\n", formatCycles(endDelete-startDelete))
}
