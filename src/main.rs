use std::io::{self, Write};
use txt2048::{Board, Dir};

fn main() {
    let mut board = Board::new(4);
    board.init();
    loop {
        print_board(&board);

        let direction = get_input();

        if !board.move_dir(direction) || !board.try_add_number() {
            println!("Better luck next time!");
        }

        if board.check_win() {
            println!("You are WINNER!!!");
            break;
        }
    }
}

fn print_board(board: &Board) {
    for (i, c) in board.get_cells().iter().enumerate() {
        print_fmt(board, *c);
        if (i + 1) % board.size() == 0 {
            println!();
            if board.get_max_num() > 99 {
                println!();
            }
        } else {
            print!(" ");
        }
    }
}

fn print_fmt(board: &Board, c: u32) {
    let digits = get_digits(board.get_max_num());
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

fn get_input() -> Dir {
    loop {
        let mut guess = String::new();

        print!("Input: ");
        io::stdout()
            .flush()
            .expect("Failed to clear IO stdout buffer!");
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
