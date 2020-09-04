use std::io;
use rand::{thread_rng, Rng};

struct Board {
    size: usize,
    cells: Vec<u32>,
    min_val: u32,   // here store available min value
    max_val: u32,   // and achieved max value
                    // based on those generator will
                    // generate next number
}

impl Board {
    fn print(&self) {
        for (i, c) in self.cells.iter().enumerate() {
            print!("{:04}", c);
            if (i + 1) % self.size == 0 {
                println!();
            } else {
                print!("-");
            }
        }
    }

    fn get_input(&self) {
        let mut guess = String::new();

        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        print!("Your input: {}", guess);
        match guess.chars().next().unwrap() {
            'h' => print!("Shift left"),
            'j' => print!("Shift down"),
            'k' => print!("Shift up"),
            'l' => print!("Shift right"),
             _  => println!("Enter one of h j k l"),
        }
    }

    fn generate(&self) -> u32 {
        let mut rng = thread_rng();
        rng.gen_range(self.min_val, self.max_val + 1)
    }

    fn get_location(&self, index: u32) -> (u32, u32) {
        if index == 0 {
            (0, 0)
        } else {
            (index / self.size as u32, index % self.size as u32)
        }
    }

    fn get_index(&self, pos: (u32, u32)) -> usize {
        let (x, y) = pos;
        y as usize * self.size + x as usize
    }

    // Returns a vector with free position indexes
    fn get_free_idx(&self) -> Vec<usize> {
        let mut indexes = Vec::new();

        for (i, x) in self.cells.iter().enumerate() {
            if *x == 0 {
                indexes.push(i);
            }
        }
        indexes
    }

    // Returns free available cell
    fn get_free_cell(&mut self) -> &mut u32 {
        let v = self.get_free_idx();
        let mut rng = thread_rng();
        let idx = rng.gen_range(0, v.len());
        &mut self.cells[v[idx]]
    }

    fn init(&mut self) {
        *self.get_free_cell() = self.generate();
        *self.get_free_cell() = self.generate();
    }
}

fn main() {
    let size = 4;
    let mut b = Board {
        size: size, // Size is 4x4
        cells: vec![0; size * size],
        min_val: 1,
        max_val: 2,
    };
    println!("Hello, world!");
    b.init();
    b.print();
    b.get_input();
}
