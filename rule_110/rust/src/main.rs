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
    let mut s = String::with_capacity((args.width + 1) * args.length);

    ca.get_current_row(&mut s);
    s.push('\n');

    for _ in 0..args.length {
        ca.next_row(&mut s);
        s.push('\n');
    }

    println!("{}", s);
}

struct CellularAutomata {
    rule: u8,
    current_row: Vec<u8>,
    next_row: Vec<u8>,
    characters: Vec<char>,
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
            characters: vec![on_character, off_character],
            width,
        };
    }

    pub fn get_current_row(&self, s: &mut String) {
        for i in &self.current_row {
            s.push(self.characters[(i == &0) as usize])
        }
    }

    pub fn next_row(&mut self, s: &mut String) {
        let mut index: u8;

        for i in 0..self.width {
            index = (4 * self.current_row[(i + self.width - 1) % self.width])
                + (2 * self.current_row[i])
                + (self.current_row[(i + 1) % self.width]);

            self.next_row[i] = (self.rule >> index) & 1;
            s.push(self.characters[self.next_row[i] as usize]);
        }

        std::mem::swap(&mut self.current_row, &mut self.next_row);
    }
}
