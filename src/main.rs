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

    fn move_left(&mut self, line_nr: usize) {
        let si = line_nr * self.size;
        let mut cn = self.cells[si];
        let mut pi = si + 0;
        for n in si + 1 .. si + self.size {
            //println!("n: {}", n);
            if self.cells[n] != 0 {
                if self.cells[n] == cn {
                    self.cells[pi] = self.update_max(2 * cn);
                    cn = 0;
                    pi = pi + 1;
                    self.cells[n] = 0;
                } else if cn == 0 {
                    cn = self.cells[n];
                    self.cells[pi] = cn;
                    self.cells[n] = 0;
                } else {
                    pi = pi + 1;
                    cn = self.cells[n];
                    self.cells[pi] = cn;
                    self.cells[n] = 0;
                }
            }
        }
    }
}

fn main() {
    let size = 4; // Size is 4x4

    let mut b = Board {
        size: size,
        cells: vec![0; size * size],
        max_num: 0
    };

    b.init();
    b.print();
    b.get_input();
}

#[cfg(test)]
mod test {
    use super::Board;

    #[test]
    fn basics() {
        let mut board = Board {
            size: 4,
            cells: vec![0; 16],
            max_num: 0,
        };

        board.init();

        // The board shall have only 2 filled cells
        let mut cnt = 0;
        let it = board.cells.iter();
        it.for_each(|x| if *x != 0 {cnt = cnt + 1; });
        assert_eq!(cnt, 2);

        // The max num may not be greater than 4
        assert!(board.max_num <= 4);
    }

    #[test]
    fn test_move_left() {
        let mut board = Board {
            size: 4,
            cells: vec![0; 16],
            max_num: 0,
        };

        let idx = board.get_index((0, 0));
        board.cells[idx] = 2;
        let idx = board.get_index((2, 0));
        board.cells[idx] = 2;
        let idx = board.get_index((0, 1));
        board.cells[idx] = 2;
        let idx = board.get_index((1, 1));
        board.cells[idx] = 2;
        let idx = board.get_index((2, 1));
        board.cells[idx] = 2;
        let idx = board.get_index((3, 1));
        board.cells[idx] = 2;
        let idx = board.get_index((1, 2));
        board.cells[idx] = 4;
        let idx = board.get_index((3, 2));
        board.cells[idx] = 2;
        let idx = board.get_index((1, 3));
        board.cells[idx] = 4;
        let idx = board.get_index((2, 3));
        board.cells[idx] = 4;

        board.max_num = 4;

        board.print();

        board.move_left(0);
//        println!();
//        board.print();

        assert!(board.max_num == 4);
        assert_eq!(board.cells[board.get_index((0, 0))], 4);
        assert_eq!(board.cells[board.get_index((1, 0))], 0);
        assert_eq!(board.cells[board.get_index((2, 0))], 0);
        assert_eq!(board.cells[board.get_index((3, 0))], 0);

        board.move_left(1);
//        println!();
//        board.print();

        assert_eq!(board.cells[board.get_index((0, 1))], 4);
        assert_eq!(board.cells[board.get_index((1, 1))], 4);
        assert_eq!(board.cells[board.get_index((2, 1))], 0);
        assert_eq!(board.cells[board.get_index((3, 1))], 0);

        board.move_left(2);
//        println!();
//        board.print();

        assert_eq!(board.cells[board.get_index((0, 2))], 4);
        assert_eq!(board.cells[board.get_index((1, 2))], 2);
        assert_eq!(board.cells[board.get_index((2, 2))], 0);
        assert_eq!(board.cells[board.get_index((3, 2))], 0);

        board.move_left(3);
//        println!();
//        board.print();

        assert!(board.max_num == 8);
        assert_eq!(board.cells[board.get_index((0, 3))], 8);
        assert_eq!(board.cells[board.get_index((1, 3))], 0);
        assert_eq!(board.cells[board.get_index((2, 3))], 0);
        assert_eq!(board.cells[board.get_index((3, 3))], 0);
    }
}
