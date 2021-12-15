use rand::{thread_rng, Rng};
use std::io::{self, Write};


pub struct Board {
    size: usize,
    cells: Vec<u32>,
    max_num: u32,
}

//#[derive(Debug)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Board {
    pub fn size(&self) -> usize {
        self.size
    }

    pub fn get_cells(&self) -> &Vec<u32> {
        &self.cells
    }

    pub fn get_max_num(&self) -> u32 {
        self.max_num
    }

    pub fn set_cell(&mut self, x: usize, y: usize, value: u32) {
        let idx = self.get_index(x, y);
        self.cells[idx] = value;
    }

    pub fn get_cell(&mut self, x: usize, y: usize) -> u32 {
        self.cells[self.get_index(x, y)]
    }

    pub fn print(&self) {
        for (i, c) in self.cells.iter().enumerate() {
            self.print_fmt(*c);
            if (i + 1) % self.size == 0 {
                println!();
                if self.max_num > 99 {
                    println!();
                }
            } else {
                print!(" ");
            }
        }
    }

    fn print_fmt(&self, c: u32) {
        let digits = Board::get_digits(self.max_num);
        match digits {
            2 => {
                print!("{:2}", c);
            }
            3 => {
                print!("{:3}", c);
            }
            4 => {
                print!("{:4}", c);
            }
            _ => {
                print!("{}", c);
            }
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

    pub fn get_input(&self) -> Dir {
        loop {
            let mut guess = String::new();

            print!("Input: ");
            io::stdout().flush().expect("Failed to flush!");
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line!");

            match guess.chars().next().unwrap() {
                'h' => return Dir::Left,
                'j' => return Dir::Down,
                'k' => return Dir::Up,
                'l' => return Dir::Right,
                _ => println!("Enter one of h/j/k/l"),
            }
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

    fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.size + x 
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
    fn get_free_cell(&mut self) -> Option<&mut u32> {
        let v = self.get_free_idx();
        if v.is_empty() {
            return None;
        }
        let mut rng = thread_rng();
        let idx = rng.gen_range(0, v.len());
        Some(&mut self.cells[v[idx]])
    }

    fn update_max(&mut self, num: u32) -> u32 {
        if num > self.max_num {
            self.max_num = num;
        }
        num
    }

    // Tries to add a number to the board
    // Returns true if attempt successed.
    // Returns false in case no space available on the board.
    pub fn try_add_number(&mut self) -> bool {
        let v = self.update_max(self.generate());
        match self.get_free_cell() {
            None => return false,
            Some(freecell) => *freecell = v,
        }
        return true;
    }

    pub fn init(&mut self) {
        self.try_add_number();
        self.try_add_number();
    }

    pub fn new(size: usize) -> Board {
        Board {
            size,
            cells: vec![0; size*size],
            max_num: 0,
        }
    }


    // returns true in case changes were made after move
    pub fn move_any<F>(&mut self, init_pos: usize, get_idx: &F) -> bool
    where
        // The closure takes line nr and index pos and returns index position
        F: Fn(usize, usize) -> usize,
    {
        let mut cn = 0;
        let mut vr = vec![];
        let mut dst = vec![0; self.size * self.size];

        dst.copy_from_slice(&self.cells[0..]);

        for n in 0..self.size {
            //println!("n: {}", n);
            let val = self.cells[get_idx(init_pos, n)];
            if cn == 0 && val != 0 {
                cn = val;
            } else {
                if cn != val && cn != 0 && val != 0 {
                    vr.push(cn);
                    cn = val;
                } else if cn == val && cn != 0 {
                    vr.push(2 * cn);
                    cn = 0;
                }
            }
        }
        vr.push(cn);
        //println!("{:?}", vr);
        for n in 0..self.size {
            let ci = get_idx(init_pos, n);
            if vr.len() > n {
                self.cells[ci] = self.update_max(vr[n]);
            } else {
                self.cells[ci] = 0;
            }
        }

        !dst.iter().zip(&self.cells).all(|(a, b)| *a == *b)
    }

    pub fn check_win(&self) -> bool {
        return self.max_num >= 2048;
    }

    pub fn check_lose(&self) -> bool {
        let size = self.size;

        let mut tmp = Board {
            size: size,
            cells: vec![0; size * size],
            max_num: self.max_num,
        };

        tmp.cells.copy_from_slice(&self.cells[0..]);
        let direct = vec![Dir::Down, Dir::Left, Dir::Right, Dir::Up];

        for d in direct {
            let move_fn: Box<dyn Fn(usize, usize) -> usize>;
            match d {
                Dir::Left => {
                    move_fn = Box::new(|base_line, index| base_line * size + index);
                }
                Dir::Right => {
                    move_fn = Box::new(|base_line, index| (1 + base_line) * size - 1 - index);
                }
                Dir::Up => {
                    move_fn = Box::new(|base_row, index| index * size + base_row);
                }
                Dir::Down => {
                    move_fn = Box::new(|base_row, index| size * (size - index - 1) + base_row);
                }
            }
            for i in 0..size {
                if tmp.move_any(i, &move_fn) {
                    return false;
                }
            }
        }
        true
    }
}


