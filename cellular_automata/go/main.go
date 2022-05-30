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
	flag.Parse()

	ca := NewCellularAutomata(rule, '0', ' ', width)

	buf := bytes.NewBuffer(make([]byte, (width+1)*length))

	buf.Write(ca.CurrentRow())
	buf.Write([]byte("\n"))

	for i := 0; i < length; i++ {
		buf.Write(ca.NextRow())
		buf.Write([]byte("\n"))
	}

	fmt.Print(buf.String())
}

type CellularAutomata struct {
	rule       byte
	currentRow []byte
	nextRow    []byte
	characters []byte
	width      int
}

func NewCellularAutomata(rule int, onCharacter, offCharacter byte, width int) *CellularAutomata {
	startingRow := make([]byte, width)
	startingRow[width/2] = 1

	return &CellularAutomata{
		rule:       uint8(rule),
		currentRow: startingRow,
		nextRow:    make([]byte, width),
		characters: []byte{offCharacter, onCharacter},
		width:      width,
	}
}

func (ca *CellularAutomata) CurrentRow() []byte {
	result := make([]byte, ca.width)

	for i := 0; i < ca.width; i++ {
		result[i] = ca.characters[ca.currentRow[i]]
	}

	return result
}

func (ca *CellularAutomata) NextRow() []byte {
	result := make([]byte, ca.width)

	var index int

	for i := 0; i < ca.width; i++ {
		index = 4*int(ca.currentRow[(i-1+ca.width)%ca.width]) +
			2*int(ca.currentRow[i]) +
			int(ca.currentRow[(i+1)%ca.width])

		ca.nextRow[i] = (ca.rule >> index) & 1
		result[i] = ca.characters[ca.nextRow[i]]
	}

	ca.currentRow, ca.nextRow = ca.nextRow, ca.currentRow

	return result
}
