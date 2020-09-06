use std::io::{self, Write};
use rand::{thread_rng, Rng};

struct Board {
    size: usize,
    cells: Vec<u32>,
    max_num: u32,
}

enum Dir {
    Left, 
    Right,
    Up,
    Down,
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

    fn get_input(&self) -> Dir {
        loop {
            let mut guess = String::new();

            print!("Input: ");
            io::stdout().flush();
            io::stdin()
                .read_line(&mut guess)
                .expect("Failed to read line!");

            match guess.chars().next().unwrap() {
                'h' => { return Dir::Left },
                'j' => { return Dir::Down },
                'k' => { return Dir::Up },
                'l' => { return Dir::Right },
                _  => println!("Enter one of h/j/k/l"),
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
    fn get_free_cell(&mut self) -> Option<&mut u32> {
        let v = self.get_free_idx();
        if v.is_empty() {
            return None
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
    fn try_add_number(&mut self) -> bool {
        let v = self.update_max(self.generate());
        match self.get_free_cell() {
            None => { return false },
            Some(freecell) => *freecell = v,
        }
        return true
    }

    fn init(&mut self) {
        self.try_add_number();
        self.try_add_number();
    }

    fn move_any<F>(&mut self, init_pos: usize, get_idx: &F) where 
        // The closure takes line nr and index pos and returns index position
        F: Fn(usize, usize) -> usize {
            let mut cn = 0;
            let mut vr = vec!();
            for n in 0 .. self.size {
                //println!("n: {}", n);
                let val = self.cells[get_idx(init_pos, n)]; 
                if cn == 0 && val != 0 {
                    cn = val;
                }
                else {
                    if cn != val && cn != 0 && val != 0 {
                        vr.push(cn);
                        cn = val;
                    } else if cn == val && cn != 0 {
                        vr.push(2*cn);
                        cn = 0;
                    }
                }
            }
            vr.push(cn);
            println!("{:?}", vr);
            for n in 0 .. self.size {
                let ci = get_idx(init_pos, n);
                if vr.len() > n {
                    self.cells[ci] = self.update_max(vr[n]);
                } else {
                    self.cells[ci] = 0;
                }
            }
        }

    fn check_win(&self) -> bool {
        return self.max_num == 2048;
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
    loop {
        b.print();
        let move_fn: Box<dyn Fn(usize, usize) -> usize>;
        match b.get_input() {
            Dir::Left => {
                move_fn = Box::new(|base_line, index| base_line * size + index);
            },
            Dir::Right => {
                move_fn = Box::new(|base_line, index| (1 + base_line) * size - 1 - index);
            },
            Dir::Up => {
                move_fn = Box::new(|base_row, index| index * size + base_row);
            },
            Dir::Down => {
                move_fn = Box::new(|base_row, index| size * (size - index - 1) + base_row);
            },
        }
        for i in 0 .. 4 {
            b.move_any(i, &move_fn);
        }
        if !b.try_add_number() {
            println!("Better luck next time!");
        }
        if b.check_win() {
            println!("You are WINNER!!!");
            break;
        }
    }
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

        let board_size = board.size;
        let move_left = |base_line, index| base_line * board_size + index;
        board.move_any(0, &move_left);
//        println!();
//        board.print();

        assert!(board.max_num == 4);
        assert_eq!(board.cells[board.get_index((0, 0))], 4);
        assert_eq!(board.cells[board.get_index((1, 0))], 0);
        assert_eq!(board.cells[board.get_index((2, 0))], 0);
        assert_eq!(board.cells[board.get_index((3, 0))], 0);

        board.move_any(1, &move_left);
//        println!();
//        board.print();

        assert_eq!(board.cells[board.get_index((0, 1))], 4);
        assert_eq!(board.cells[board.get_index((1, 1))], 4);
        assert_eq!(board.cells[board.get_index((2, 1))], 0);
        assert_eq!(board.cells[board.get_index((3, 1))], 0);

        board.move_any(2, &move_left);
//        println!();
//        board.print();

        assert_eq!(board.cells[board.get_index((0, 2))], 4);
        assert_eq!(board.cells[board.get_index((1, 2))], 2);
        assert_eq!(board.cells[board.get_index((2, 2))], 0);
        assert_eq!(board.cells[board.get_index((3, 2))], 0);

        board.move_any(3, &move_left);
//        println!();
//        board.print();

        assert!(board.max_num == 8);
        assert_eq!(board.cells[board.get_index((0, 3))], 8);
        assert_eq!(board.cells[board.get_index((1, 3))], 0);
        assert_eq!(board.cells[board.get_index((2, 3))], 0);
        assert_eq!(board.cells[board.get_index((3, 3))], 0);
    }

    #[test]
    fn test_move_right() {
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

        let board_size = board.size;
        let move_right = |base_line, index| (1 + base_line) * board_size - 1 - index;
        board.move_any(0, &move_right);
        //board.move_right(0);
        println!();
        board.print();

        assert!(board.max_num == 4);
        assert_eq!(board.cells[board.get_index((0, 0))], 0);
        assert_eq!(board.cells[board.get_index((1, 0))], 0);
        assert_eq!(board.cells[board.get_index((2, 0))], 0);
        assert_eq!(board.cells[board.get_index((3, 0))], 4);

        board.move_any(1, &move_right);
        println!();
        board.print();

        assert_eq!(board.cells[board.get_index((0, 1))], 0);
        assert_eq!(board.cells[board.get_index((1, 1))], 0);
        assert_eq!(board.cells[board.get_index((2, 1))], 4);
        assert_eq!(board.cells[board.get_index((3, 1))], 4);

        board.move_any(2, &move_right);
        println!();
        board.print();

        assert_eq!(board.cells[board.get_index((0, 2))], 0);
        assert_eq!(board.cells[board.get_index((1, 2))], 0);
        assert_eq!(board.cells[board.get_index((2, 2))], 4);
        assert_eq!(board.cells[board.get_index((3, 2))], 2);

        board.move_any(3, &move_right);
        println!();
        board.print();

        assert!(board.max_num == 8);
        assert_eq!(board.cells[board.get_index((0, 3))], 0);
        assert_eq!(board.cells[board.get_index((1, 3))], 0);
        assert_eq!(board.cells[board.get_index((2, 3))], 0);
        assert_eq!(board.cells[board.get_index((3, 3))], 8);
    }

    #[test]
    fn test_move_down() {
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

        let board_size = board.size;
        let move_down = |base_row, index| board_size * (board_size - index - 1) + base_row;
        
        board.move_any(0, &move_down);
        println!();
        board.print();

        assert!(board.max_num == 4);
        assert_eq!(board.cells[board.get_index((0, 0))], 0);
        assert_eq!(board.cells[board.get_index((0, 1))], 0);
        assert_eq!(board.cells[board.get_index((0, 2))], 0);
        assert_eq!(board.cells[board.get_index((0, 3))], 4);

        board.move_any(1, &move_down);
        println!();
        board.print();

        assert!(board.max_num == 8);
        assert_eq!(board.cells[board.get_index((1, 0))], 0);
        assert_eq!(board.cells[board.get_index((1, 1))], 0);
        assert_eq!(board.cells[board.get_index((1, 2))], 2);
        assert_eq!(board.cells[board.get_index((1, 3))], 8);

        board.move_any(2, &move_down);
        println!();
        board.print();

        assert_eq!(board.cells[board.get_index((2, 0))], 0);
        assert_eq!(board.cells[board.get_index((2, 1))], 0);
        assert_eq!(board.cells[board.get_index((2, 2))], 4);
        assert_eq!(board.cells[board.get_index((2, 3))], 4);

        board.move_any(3, &move_down);
        println!();
        board.print();

        assert!(board.max_num == 8);
        assert_eq!(board.cells[board.get_index((3, 0))], 0);
        assert_eq!(board.cells[board.get_index((3, 1))], 0);
        assert_eq!(board.cells[board.get_index((3, 2))], 0);
        assert_eq!(board.cells[board.get_index((3, 3))], 4);
    }

    #[test]
    fn test_move_up() {
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

        let board_size = board.size;
        let move_up = |base_row, index| index*board_size + base_row;
        
        board.move_any(0, &move_up);
        println!();
        board.print();

        assert!(board.max_num == 4);
        assert_eq!(board.cells[board.get_index((0, 0))], 4);
        assert_eq!(board.cells[board.get_index((0, 1))], 0);
        assert_eq!(board.cells[board.get_index((0, 2))], 0);
        assert_eq!(board.cells[board.get_index((0, 3))], 0);

        board.move_any(1, &move_up);
        println!();
        board.print();

        assert!(board.max_num == 8);
        assert_eq!(board.cells[board.get_index((1, 0))], 2);
        assert_eq!(board.cells[board.get_index((1, 1))], 8);
        assert_eq!(board.cells[board.get_index((1, 2))], 0);
        assert_eq!(board.cells[board.get_index((1, 3))], 0);

        board.move_any(2, &move_up);
        println!();
        board.print();

        assert_eq!(board.cells[board.get_index((2, 0))], 4);
        assert_eq!(board.cells[board.get_index((2, 1))], 4);
        assert_eq!(board.cells[board.get_index((2, 2))], 0);
        assert_eq!(board.cells[board.get_index((2, 3))], 0);

        board.move_any(3, &move_up);
        println!();
        board.print();

        assert!(board.max_num == 8);
        assert_eq!(board.cells[board.get_index((3, 0))], 4);
        assert_eq!(board.cells[board.get_index((3, 1))], 0);
        assert_eq!(board.cells[board.get_index((3, 2))], 0);
        assert_eq!(board.cells[board.get_index((3, 3))], 0);
    }

    #[test]
//  2-0-0-0
//  0-0-0-0
//  8-0-0-0
//  4-0-0-0
//  Input: j
//  0-0-0-0
//  2-0-0-0
//  0-0-2-0
//  4-0-0-0
    fn test_case1() {
        let mut board = Board {
            size: 4,
            cells: vec![0; 16],
            max_num: 0,
        };

        let idx = board.get_index((0, 0));
        board.cells[idx] = 2;
        let idx = board.get_index((0, 2));
        board.cells[idx] = 8;
        let idx = board.get_index((0, 3));
        board.cells[idx] = 4;
        board.max_num = 8;

        board.print();
        let board_size = board.size;
        let move_down = |base_row, index| board_size * (board_size - index - 1) + base_row;
        for i in 0 .. 4 {
            board.move_any(i, &move_down);
        }

        println!();
        board.print();

        assert!(board.max_num == 8);
        assert_eq!(board.cells[board.get_index((0, 0))], 0);
        assert_eq!(board.cells[board.get_index((0, 1))], 2);
        assert_eq!(board.cells[board.get_index((0, 2))], 8);
        assert_eq!(board.cells[board.get_index((0, 3))], 4);

        assert_eq!(board.cells[board.get_index((2, 2))], 0);
    }

}
