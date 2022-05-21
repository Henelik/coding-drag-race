use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    // the character width of the simulation
    #[clap(short, long, default_value_t = 128)]
    width: usize,

    // the number of rows of the simulation
    #[clap(short, long, default_value_t = 128)]
    length: usize,

    // the Elementary Cellular Automata rule to use (must be 0-255)
    #[clap(short, long, default_value_t = 110)]
    rule: u8,
}

fn main() {
    let args = Args::parse();
    let mut ca = CellularAutomata::new(args.rule, '0', ' ', args.width);
    let mut s = String::new();

    ca.get_current_row(&mut s);
    println!("{}", s);

    for _ in 0..args.length {
        s.clear();
        ca.next_row(&mut s);
        println!("{}", s)
    }
}

struct CellularAutomata {
    rule: u8,
    current_row: Vec<u8>,
    next_row: Vec<u8>,
    on_character: char,
    off_character: char,
    width: usize,
}

impl CellularAutomata {
    pub fn new(rule: u8, on_character: char, off_character: char, width: usize) -> Self {
        let mut first_row = vec![0; width];
        first_row[width / 2] = 1;

        return Self {
            rule,
            current_row: first_row,
            next_row: vec![0; width],
            on_character,
            off_character,
            width,
        };
    }

    pub fn get_current_row(&self, s: &mut String) {
        for i in &self.current_row {
            if i == &0 {
                s.push(self.off_character)
            } else {
                s.push(self.on_character)
            }
        }
    }

    pub fn next_row(&mut self, s: &mut String) {
        let mut index: u32;

        for i in 0..self.width {
            index = 0;

            if i != 0 && self.current_row[i - 1] != 0 {
                index += 4;
            }

            if self.current_row[i] != 0 {
                index += 2;
            }

            if i != self.width - 1 && self.current_row[i + 1] != 0 {
                index += 1;
            }

            if (self.rule >> index) & 1 == 1 {
                self.next_row[i] = 1;
                s.push(self.on_character)
            } else {
                self.next_row[i] = 0;
                s.push(self.off_character)
            }
        }

        std::mem::swap(&mut self.current_row, &mut self.next_row);
    }
}
