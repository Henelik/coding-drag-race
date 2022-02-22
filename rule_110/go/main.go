package main

import (
	"bytes"
	"flag"
	"fmt"
)

var width int
var length int
var rule int

func init() {
	flag.IntVar(&width, "width", 128, "the character width of the simulation")
	flag.IntVar(&length, "length", 128, "the number of rows of the simulation")
	flag.IntVar(&rule, "rule", 110, "the Elementary Cellular Automata rule to use (must be 0-255)")
}

func main() {
	startingRow := make([]bool, width)
	startingRow[width/2] = true

	ca := NewCellularAutomata(rule, '0', ' ', width)

	buf := bytes.NewBuffer(make([]byte, (width+2)*length))

	buf.Write(ca.CurrentRow())
	buf.Write([]byte("\n"))

	for i := 0; i < length; i++ {
		buf.Write(ca.NextRow())
		buf.Write([]byte("\n"))
	}

	fmt.Print(buf.String())
}

type CellularAutomata struct {
	rule         byte
	currentRow   []bool
	onCharacter  byte
	offCharacter byte
	width        int
}

func NewCellularAutomata(rule int, onCharacter, offCharacter byte, width int) *CellularAutomata {
	startingRow := make([]bool, width)
	startingRow[width/2] = true

	return &CellularAutomata{
		rule:         uint8(rule),
		currentRow:   startingRow,
		onCharacter:  onCharacter,
		offCharacter: offCharacter,
		width:        width,
	}
}

func (ca *CellularAutomata) CurrentRow() []byte {
	result := make([]byte, ca.width)

	for i := 0; i < ca.width; i++ {
		if ca.currentRow[i] {
			result[i] = ca.onCharacter
		} else {
			result[i] = ca.offCharacter
		}
	}

	return result
}

func (ca *CellularAutomata) NextRow() []byte {
	result := make([]byte, ca.width)
	nextRow := make([]bool, ca.width)

	var index int

	for i := 0; i < ca.width; i++ {
		index = 0

		if i != 0 && ca.currentRow[i-1] {
			index += 4
		}
		if ca.currentRow[i] {
			index += 2
		}
		if i != ca.width-1 && ca.currentRow[i+1] {
			index += 1
		}

		if (ca.rule>>index)&1 == 1 {
			result[i] = ca.onCharacter
			nextRow[i] = true
		} else {
			result[i] = ca.offCharacter
			nextRow[i] = false
		}
	}

	ca.currentRow = nextRow

	return result
}
