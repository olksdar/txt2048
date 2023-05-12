use rand::{thread_rng, Rng};

pub struct Board {
    size: usize,
    cells: Vec<u32>,
    max_num: u32,
}

pub trait ExtModify {
    fn set_cell(&mut self, x: usize, y: usize, value: u32);
}

impl ExtModify for Board {
    fn set_cell(&mut self, x: usize, y: usize, value: u32) {
        let idx = self.get_index(x, y);
        self.cells[idx] = value;
    }
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

    pub fn get_cell(&mut self, x: usize, y: usize) -> u32 {
        self.cells[self.get_index(x, y)]
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

    pub fn check_game_over(&self) -> bool {
        // If no space left and no adjacent blocks are the same then game is over
        let check_adj_left = |index | if index % self.size == 0 {false} else {self.cells[index] == self.cells[index - 1]};
        let check_adj_up = |index| if index < self.size {false} else {self.cells[index] == self.cells[index - self.size]};

        for idx in 0..(self.size*self.size) {
            if self.cells[idx] == 0 || check_adj_left(idx) || check_adj_up(idx) { 
                return false;
            }
        };
        true
    }
}


