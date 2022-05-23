package main

import (
	"testing"
)

func BenchmarkCellularAutomata_CurrentRow(b *testing.B) {
	b.StopTimer()

	width = 1024

	ca := NewCellularAutomata(30, '0', ' ', width)
	bytes := make([]byte, width)

	b.StartTimer()

	for i := 0; i < b.N; i++ {
		bytes = ca.NextRow()
	}

	bytes = bytes[:]
}
