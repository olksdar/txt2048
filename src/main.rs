use std::io;

struct Board {
    size: u8,
    cells: Vec<u16>,
}

impl Board {
    fn print(&self) {
        let mut i: u8 = 0;
        for c in &self.cells {
            i = i+1;
            print!("{:04}", c);
            if i % self.size == 0 {
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
             _  => print!("Enter one of h j k l"),
        }
    }
}

fn main() {
    let size = 4;
    let b = Board {
        size: size, // Size is 4x4
        cells: vec![0; (size * size).into()],
    };
    println!("Hello, world!");
    b.print();
    b.get_input();
}
