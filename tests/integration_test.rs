use txt2048::Board as Board;

#[test]
fn basics() {
    let mut board = Board::new(4);
    board.init();

    // The board shall have only 2 filled cells
    let mut cnt = 0;
    let it = board.get_cells().iter();
    it.for_each(|x| {
        if *x != 0 {
            cnt = cnt + 1;
        }
    });
    assert_eq!(cnt, 2);

    // The max num may not be greater than 4
    assert!(board.get_max_num() <= 4);
}

#[test]
fn test_move_left() {
    let mut board = Board::new(4);

    board.set_cell(0, 0, 2);
    board.set_cell(2, 0, 2);
    board.set_cell(0, 1, 2);
    board.set_cell(1, 1, 2);
    board.set_cell(2, 1, 2);
    board.set_cell(3, 1, 2);
    board.set_cell(1, 2, 4);
    board.set_cell(3, 2, 2);
    board.set_cell(1, 3, 4);
    board.set_cell(2, 3, 4);

    //board.get_max_num() = 4;

 //   board.print();

    let board_size = board.size();
    let move_left = |base_line, index| base_line * board_size + index;
    board.move_any(0, &move_left);
    //        println!();
    //        board.print();

    assert!(board.get_max_num() == 4);
    assert_eq!(board.get_cell(0, 0),  4);
    assert_eq!(board.get_cell(1, 0),  0);
    assert_eq!(board.get_cell(2, 0),  0);
    assert_eq!(board.get_cell(3, 0),  0);

    board.move_any(1, &move_left);
    //        println!();
    //        board.print();

    assert_eq!(board.get_cell(0, 1),  4);
    assert_eq!(board.get_cell(1, 1),  4);
    assert_eq!(board.get_cell(2, 1),  0);
    assert_eq!(board.get_cell(3, 1),  0);

    board.move_any(2, &move_left);
    //        println!();
    //        board.print();

    assert_eq!(board.get_cell(0, 2),  4);
    assert_eq!(board.get_cell(1, 2),  2);
    assert_eq!(board.get_cell(2, 2),  0);
    assert_eq!(board.get_cell(3, 2),  0);

    board.move_any(3, &move_left);
    //        println!();
    //        board.print();

    assert!(board.get_max_num() == 8);
    assert_eq!(board.get_cell(0, 3),  8);
    assert_eq!(board.get_cell(1, 3),  0);
    assert_eq!(board.get_cell(2, 3),  0);
    assert_eq!(board.get_cell(3, 3),  0);
}

#[test]
fn test_move_right() {
    let mut board = Board::new(4);
 
    board.set_cell(0, 0, 2);
    board.set_cell(2, 0, 2);
    board.set_cell(0, 1, 2);
    board.set_cell(1, 1, 2);
    board.set_cell(2, 1, 2);
    board.set_cell(3, 1, 2);
    board.set_cell(1, 2, 4);
    board.set_cell(3, 2, 2);
    board.set_cell(1, 3, 4);
    board.set_cell(2, 3, 4);
//    board.print();

    let board_size = board.size();
    let move_right = |base_line, index| (1 + base_line) * board_size - 1 - index;
    board.move_any(0, &move_right);
    // println!();
    // board.print();

    assert!(board.get_max_num() == 4);
    assert_eq!(board.get_cell(0, 0),  0);
    assert_eq!(board.get_cell(1, 0),  0);
    assert_eq!(board.get_cell(2, 0),  0);
    assert_eq!(board.get_cell(3, 0),  4);

    board.move_any(1, &move_right);
    // println!();
    // board.print();
    assert_eq!(board.get_cell(0, 1),  0);
    assert_eq!(board.get_cell(1, 1),  0);
    assert_eq!(board.get_cell(2, 1),  4);
    assert_eq!(board.get_cell(3, 1),  4);

    board.move_any(2, &move_right);
    // println!();
    // board.print();
    assert_eq!(board.get_cell(0, 2),  0);
    assert_eq!(board.get_cell(1, 2),  0);
    assert_eq!(board.get_cell(2, 2),  4);
    assert_eq!(board.get_cell(3, 2),  2);

    board.move_any(3, &move_right);
    // println!();
    // board.print();
    assert_eq!(board.get_cell(0, 3),  0);
    assert_eq!(board.get_cell(1, 3),  0);
    assert_eq!(board.get_cell(2, 3),  0);
    assert_eq!(board.get_cell(3, 3),  8);

    assert!(board.get_max_num() == 8);
}


#[test]
fn test_move_down() {
    let mut board = Board::new(4);
    board.set_cell(0, 0, 2);
    board.set_cell(2, 0, 2);
    board.set_cell(0, 1, 2);
    board.set_cell(1, 1, 2);
    board.set_cell(2, 1, 2);
    board.set_cell(3, 1, 2);
    board.set_cell(1, 2, 4);
    board.set_cell(3, 2, 2);
    board.set_cell(1, 3, 4);
    board.set_cell(2, 3, 4);

    let board_size = board.size();
    let move_down = |base_row, index| board_size * (board_size - index - 1) + base_row;

    board.move_any(0, &move_down);
    // println!();
    // board.print();

    assert!(board.get_max_num()== 4);
    assert_eq!(board.get_cell(0, 0),  0);
    assert_eq!(board.get_cell(0, 1),  0);
    assert_eq!(board.get_cell(0, 2),  0);
    assert_eq!(board.get_cell(0, 3),  4);

    board.move_any(1, &move_down);
    // println!();
    // board.print();

    assert!(board.get_max_num() == 8);
    assert_eq!(board.get_cell(1, 0),  0);
    assert_eq!(board.get_cell(1, 1),  0);
    assert_eq!(board.get_cell(1, 2),  2);
    assert_eq!(board.get_cell(1, 3),  8);

    board.move_any(2, &move_down);
    // println!();
    // board.print();
    assert_eq!(board.get_cell(2, 0),  0);
    assert_eq!(board.get_cell(2, 1),  0);
    assert_eq!(board.get_cell(2, 2),  4);
    assert_eq!(board.get_cell(2, 3),  4);

    board.move_any(3, &move_down);
    // println!();
    // board.print();
    assert_eq!(board.get_cell(3, 0),  0);
    assert_eq!(board.get_cell(3, 1),  0);
    assert_eq!(board.get_cell(3, 2),  0);
    assert_eq!(board.get_cell(3, 3),  4);
    assert!(board.get_max_num() == 8);
}

#[test]
fn test_move_up() {
    let mut board = Board::new(4);
    board.set_cell(0, 0, 2);
    board.set_cell(2, 0, 2);
    board.set_cell(0, 1, 2);
    board.set_cell(1, 1, 2);
    board.set_cell(2, 1, 2);
    board.set_cell(3, 1, 2);
    board.set_cell(1, 2, 4);
    board.set_cell(3, 2, 2);
    board.set_cell(1, 3, 4);
    board.set_cell(2, 3, 4);

    // board.print();

    let board_size = board.size();
    let move_up = |base_row, index| index * board_size + base_row;
    board.move_any(0, &move_up);
    // println!();
    // board.print();

    assert!(board.get_max_num()== 4);
    assert_eq!(board.get_cell(0, 0),  4);
    assert_eq!(board.get_cell(0, 1),  0);
    assert_eq!(board.get_cell(0, 2),  0);
    assert_eq!(board.get_cell(0, 3),  0);

    board.move_any(1, &move_up);
    // println!();
    // board.print();

    assert!(board.get_max_num() == 8);
    assert_eq!(board.get_cell(1, 0),  2);
    assert_eq!(board.get_cell(1, 1),  8);
    assert_eq!(board.get_cell(1, 2),  0);
    assert_eq!(board.get_cell(1, 3),  0);

    board.move_any(2, &move_up);
    // println!();
    // board.print();
    assert_eq!(board.get_cell(2, 0),  4);
    assert_eq!(board.get_cell(2, 1),  4);
    assert_eq!(board.get_cell(2, 2),  0);
    assert_eq!(board.get_cell(2, 3),  0);

    board.move_any(3, &move_up);
    // println!();
    // board.print();

    assert!(board.get_max_num()== 8);
    assert_eq!(board.get_cell(3, 0),  4);
    assert_eq!(board.get_cell(3, 1),  0);
    assert_eq!(board.get_cell(3, 2),  0);
    assert_eq!(board.get_cell(3, 3),  0);
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
    let mut board = Board::new(4);
    board.set_cell(0, 0, 2);
    board.set_cell(0, 2, 8);
    board.set_cell(0, 3, 4);

    // board.print();
    let board_size = board.size();
    let move_down = |base_row, index| board_size * (board_size - index - 1) + base_row;
    for i in 0..4 {
        board.move_any(i, &move_down);
    }

    // println!();
    // board.print();

    assert!(board.get_max_num() == 8);
    assert_eq!(board.get_cell(0, 0),  0);
    assert_eq!(board.get_cell(0, 1),  2);
    assert_eq!(board.get_cell(0, 2),  8);
    assert_eq!(board.get_cell(0, 3),  4);
    assert_eq!(board.get_cell(2, 2),  0);
}

#[test]
//  0-0-2-0
//  0-0-0-0
//  0-0-0-0
//  0-0-4-0
//  Input: l
//  0-0-0-2
//  0-0-0-0
//  0-0-0-0
//  0-0-0-4
fn test_case2() {
    let mut board = Board::new(4);
    board.set_cell(2, 0, 2);
    board.set_cell(2, 3, 4);
   
    // board.print();
    let board_size = board.size();
    let move_right = |base_line, index| (1 + base_line) * board_size - 1 - index;
    for i in 0..4 {
        board.move_any(i, &move_right);
    }

    // println!();
    // board.print();
    assert_eq!(board.get_cell(3, 0),  2);
    assert_eq!(board.get_cell(3, 3),  4);
    assert_eq!(board.get_cell(2, 0),  0);
    assert_eq!(board.get_cell(2, 3),  0);
}

#[test]
fn test_loose() {
    let mut board = Board::new(4);
    board.set_cell(0, 0, 2);
    board.set_cell(0, 1, 64);
    board.set_cell(0, 2, 2);
    board.set_cell(0, 3, 8);
    board.set_cell(1, 0, 8);
    board.set_cell(1, 1, 16);
    board.set_cell(1, 2, 64);
    board.set_cell(1, 3, 2);
    board.set_cell(2, 0, 4);
    board.set_cell(2, 1, 64);
    board.set_cell(2, 2, 32);
    board.set_cell(2, 3, 8);
    board.set_cell(3, 0, 8);
    board.set_cell(3, 1, 2);
    board.set_cell(3, 2, 4);
    board.set_cell(3, 3, 2);

    assert!(board.check_lose())
}

#[test]
fn test_loose1() {
    let mut board = Board::new(4);
    board.set_cell(2, 0, 2);
    board.set_cell(2, 3, 4);

    assert!(!board.check_lose());
}
