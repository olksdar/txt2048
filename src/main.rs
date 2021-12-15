
use txt2048::Board;
use txt2048::Dir;

fn main() {
    let size = 4; // Size is 4x4

    let mut b = Board::new(4); 
    b.init();
    loop {
        b.print();
        if b.check_lose() {
            println!("Better luck next time!");
            break;
        }
        let move_fn: Box<dyn Fn(usize, usize) -> usize>;
        match b.get_input() {
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
        let mut changed = false;
        for i in 0..b.size() {
            let res = b.move_any(i, &move_fn);
            changed = changed || res;
        }
        //print!("good {}", changed);
        if changed {
            if !b.try_add_number() {
                println!("Better luck next time!");
            }
        }
        if b.check_win() {
            println!("You are WINNER!!!");
            break;
        }
    }
}

