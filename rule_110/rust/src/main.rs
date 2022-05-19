extern crate core;

fn main() {
    let length = 128;
    let mut ca = CellularAutomata::new(110, '0', ' ', 128);
    let mut s = String::new();

    ca.get_current_row(&mut s);
    println!("{}", s);

    for _ in 0..length {
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
    pub fn new(rule: u8, on_character: char, off_character: char, width: usize) -> Self{
        let mut first_row = vec![0; width];
        first_row[width / 2] = 1;

        return Self {
            rule,
            current_row: first_row,
            next_row: vec![0; width],
            on_character,
            off_character,
            width,
        }
    }

    pub fn get_current_row(&self, s: &mut String) {
        for i in &self.current_row{
            if i == &0 {
                s.push(self.off_character)
            }
            else {
                s.push(self.on_character)
            }
        }
    }

    pub fn next_row(&mut self, s: &mut String) {
        let mut index :u32;

        for i in 0..self.width {
            index = 0;

            if i != 0 && self.current_row[i - 1] != 0 {
                index += 4;
            }

            if self.current_row[i] != 0 {
                index += 2;
            }

            if i != self.width.clone()-1 && self.current_row[i.clone()+1] != 0 {
                index += 1;
            }

            if (self.rule.clone()>>index)&1 == 1 {
                self.next_row[i.clone()] = 1;
                s.push(self.on_character)
            }
            else {
                self.next_row[i.clone() as usize] = 0;
                s.push(self.off_character)
            }
        }

        std::mem::swap(&mut self.current_row, &mut self.next_row);
    }
}
