use rand::{thread_rng, Rng};
use std::cmp;

pub struct Board {
    size: usize,
    cells: Vec<u32>,
    max_num: u32,
}

#[derive(Debug)]
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

    // Wrapper that updates board maximum value
    // returns original value, passed to num
    fn update_max(&mut self, num: u32) -> u32 {
        self.max_num = cmp::max(num, self.max_num);
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
        // We have 2 numbers on board initially
        self.try_add_number();
        self.try_add_number();
    }

    pub fn new(size: usize) -> Board {
        Board {
            size,
            cells: vec![0; size * size],
            max_num: 0,
        }
    }

    //
    // fn move_any: shifts Board cells using iterator function get_idx
    // If two adjacent cells has same value then they are changed by one cell with value equal
    // to double of former value.
    //
    // @return: true in case changes were made after move i.e. not game end.
    // In case no moves are available returns false, i.e. game end.
    //
    // @param get_idx: function that converts start position and index into appropriate position in the
    // array so it acts like an iterator. For example call(start_row, i) for i: 0..3 will return start_row,
    // start_raw + 1, start_row + 2, start_row + 3.
    // @param init_pos: start position to iterate from
    //
    fn move_any<F>(&mut self, init_pos: usize, get_idx: &F) -> bool
    where
        // The closure takes line nr and index pos and returns index position
        F: Fn(usize, usize, usize) -> usize,
    {
        let mut cn = 0;
        let mut vr = vec![];
        // The temporary array to perform verification if original array was changed
        let mut dst = vec![0; self.size * self.size];

        dst.copy_from_slice(&self.cells[0..]);

        for n in 0..self.size {
            //println!("n: {}", n);
            // get_idx is used to achieve n-th element from Board
            let val = self.cells[get_idx(self.size, init_pos, n)];
            // Initialize cn with value from Board
            if cn == 0 && val != 0 {
                cn = val;
            } else {
                // in case cn is not adjacent with value add it to storage array vr
                // and store new value as cn
                if cn != val && cn != 0 && val != 0 {
                    vr.push(cn);
                    cn = val;
                // in case cn is adjacent with value, add doubled cn to storage array and clear cn
                } else if cn == val && cn != 0 {
                    vr.push(2 * cn);
                    cn = 0;
                }
            }
        }
        // add latest cn to storage array
        vr.push(cn);
        //println!("{:?}", vr);

        // update every cell from original Board with value from storage array
        for n in 0..self.size {
            let ci = get_idx(self.size, init_pos, n);
            if vr.len() > n {
                // update max value as well
                self.cells[ci] = self.update_max(vr[n]);
            } else {
                self.cells[ci] = 0;
            }
        }

        !dst.iter().zip(&self.cells).all(|(a, b)| *a == *b)
    }

    // Updates board with shifting all the numbers at direction side
    // Returns true if shift happened
    pub fn move_dir(&mut self, direction: Dir) -> bool {
        // returns index based on direction
        let get_index: fn(size: usize, base_line: usize, index: usize) -> usize = match direction {
            Dir::Left => |size, base_line, index| base_line * size + index,
            Dir::Right => |size, base_line, index| (1 + base_line) * size - 1 - index,
            Dir::Up =>
            // base line in this case is base row index
                |size, base_line, index| index * size + base_line
            ,
            // same as above
            Dir::Down => |size, base_line, index| size * (size - index - 1) + base_line,
        };

        let mut shift_result = false;
        // Use move_fn over every line on board.
        for idx in 0..self.size {
            shift_result |= self.move_any(idx, &get_index);
        }

        shift_result
    }

    pub fn check_win(&self) -> bool {
        return self.max_num >= 2048;
    }
}
