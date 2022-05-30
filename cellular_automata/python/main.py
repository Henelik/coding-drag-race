import argparse


class CellularAutomata:
    def __init__(self, rule, on_character, off_character, width):
        self.rule = rule
        self.current_row = [off_character] * width
        self.next_row = [off_character] * width
        self.on_character = on_character
        self.off_character = off_character
        self.width = width

        self.current_row[width // 2] = on_character

    def generate_next_row(self):
        for i in range(self.width):
            index = 0

            if self.current_row[(i - 1) % self.width] == self.on_character:
                index += 4
            if self.current_row[i] == self.on_character:
                index += 2
            if self.current_row[(i + 1) % self.width] == self.on_character:
                index += 1

            if (self.rule >> index) & 1 == 1:
                self.next_row[i] = self.on_character
            else:
                self.next_row[i] = self.off_character

        self.next_row, self.current_row = self.current_row, self.next_row


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("-w", "--width", default=128, help="the character width of the simulation", type=int)
    parser.add_argument("-l", "--length", default=128, help="the number of rows of the simulation", type=int)
    parser.add_argument("-r", "--rule", default=110, help="the Elementary Cellular Automata rule to use (must be 0-255)", type=int)
    arguments = parser.parse_args()

    ca = CellularAutomata(arguments.rule, "0", " ", arguments.width)

    for i in range(arguments.length):
        print("".join(ca.current_row))
        ca.generate_next_row()
