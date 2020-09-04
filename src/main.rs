use std::io::{self, Write};
use rand::{thread_rng, Rng};

struct Board {
    size: usize,
    cells: Vec<u32>,
    max_num: u32,
}

impl Board {
    fn print(&self) {
        for (i, c) in self.cells.iter().enumerate() {
            self.print_fmt(*c);
            if (i + 1) % self.size == 0 {
                println!();
            } else {
                print!("-");
            }
        }
    }

    fn print_fmt(&self, c: u32) {
        let digits = Board::get_digits(self.max_num);
        match digits {
            2 => { print!("{:02}", c); },
            3 => { print!("{:03}", c); },
            4 => { print!("{:04}", c); },
            _ => { print!("{}", c); },
        };
    }

    // Returns how many digits has the number in dec
    fn get_digits(num: u32) -> u32 {
        let mut n = num;
        let mut cnt = 0;
        while n != 0 {
            n = n / 10;
            cnt = cnt + 1;
        }
        cnt
    }

    fn get_input(&self) {
        let mut guess = String::new();

        print!("Input: ");
        io::stdout().flush();
        io::stdin()
            .read_line(&mut guess)
            .expect("Failed to read line!");

        match guess.chars().next().unwrap() {
            'h' => print!("Shift left"),
            'j' => print!("Shift down"),
            'k' => print!("Shift up"),
            'l' => print!("Shift right"),
             _  => println!("Enter one of h/j/k/l"),
        }
    }

    fn generate(&self) -> u32 {
        let mut rng = thread_rng();
        match rng.gen_range(0, 2) as u8 {
            0 => 2,
            1 => 4,
            _ => 8,
        }
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

    fn update_max(&mut self, num: u32) -> u32 {
        if num > self.max_num {
            self.max_num = num;
        }
        num
    }

    fn init(&mut self) {
        *self.get_free_cell() = self.update_max(self.generate());
        *self.get_free_cell() = self.update_max(self.generate());
    }
}

fn main() {
    let size = 4;
    let mut b = Board {
        size: size, // Size is 4x4
        cells: vec![0; size * size],
        max_num: 0
    };

    b.init();
    b.print();
    b.get_input();
}
