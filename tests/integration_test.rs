use txt2048::Board;
use txt2048::Dir;

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
fn move_left() {
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

    board.move_dir(Dir::Left);

    assert_eq!(board.get_cell(0, 0), 4);
    assert_eq!(board.get_cell(1, 0), 0);
    assert_eq!(board.get_cell(2, 0), 0);
    assert_eq!(board.get_cell(3, 0), 0);

    assert_eq!(board.get_cell(0, 1), 4);
    assert_eq!(board.get_cell(1, 1), 4);
    assert_eq!(board.get_cell(2, 1), 0);
    assert_eq!(board.get_cell(3, 1), 0);

    assert_eq!(board.get_cell(0, 2), 4);
    assert_eq!(board.get_cell(1, 2), 2);
    assert_eq!(board.get_cell(2, 2), 0);
    assert_eq!(board.get_cell(3, 2), 0);

    assert_eq!(board.get_cell(0, 3), 8);
    assert_eq!(board.get_cell(1, 3), 0);
    assert_eq!(board.get_cell(2, 3), 0);
    assert_eq!(board.get_cell(3, 3), 0);

    assert!(board.get_max_num() == 8);
}

#[test]
fn move_right() {
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

    board.move_dir(Dir::Right);

    assert_eq!(board.get_cell(0, 0), 0);
    assert_eq!(board.get_cell(1, 0), 0);
    assert_eq!(board.get_cell(2, 0), 0);
    assert_eq!(board.get_cell(3, 0), 4);

    assert_eq!(board.get_cell(0, 1), 0);
    assert_eq!(board.get_cell(1, 1), 0);
    assert_eq!(board.get_cell(2, 1), 4);
    assert_eq!(board.get_cell(3, 1), 4);

    assert_eq!(board.get_cell(0, 2), 0);
    assert_eq!(board.get_cell(1, 2), 0);
    assert_eq!(board.get_cell(2, 2), 4);
    assert_eq!(board.get_cell(3, 2), 2);

    assert_eq!(board.get_cell(0, 3), 0);
    assert_eq!(board.get_cell(1, 3), 0);
    assert_eq!(board.get_cell(2, 3), 0);
    assert_eq!(board.get_cell(3, 3), 8);

    assert!(board.get_max_num() == 8);
}

#[test]
fn move_down() {
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

    board.move_dir(Dir::Down);

    assert_eq!(board.get_cell(0, 0), 0);
    assert_eq!(board.get_cell(0, 1), 0);
    assert_eq!(board.get_cell(0, 2), 0);
    assert_eq!(board.get_cell(0, 3), 4);

    assert_eq!(board.get_cell(1, 0), 0);
    assert_eq!(board.get_cell(1, 1), 0);
    assert_eq!(board.get_cell(1, 2), 2);
    assert_eq!(board.get_cell(1, 3), 8);

    assert_eq!(board.get_cell(2, 0), 0);
    assert_eq!(board.get_cell(2, 1), 0);
    assert_eq!(board.get_cell(2, 2), 4);
    assert_eq!(board.get_cell(2, 3), 4);

    assert_eq!(board.get_cell(3, 0), 0);
    assert_eq!(board.get_cell(3, 1), 0);
    assert_eq!(board.get_cell(3, 2), 0);
    assert_eq!(board.get_cell(3, 3), 4);

    assert!(board.get_max_num() == 8);
}

#[test]
fn move_up() {
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

    board.move_dir(Dir::Up);

    assert_eq!(board.get_cell(0, 0), 4);
    assert_eq!(board.get_cell(0, 1), 0);
    assert_eq!(board.get_cell(0, 2), 0);
    assert_eq!(board.get_cell(0, 3), 0);

    assert_eq!(board.get_cell(1, 0), 2);
    assert_eq!(board.get_cell(1, 1), 8);
    assert_eq!(board.get_cell(1, 2), 0);
    assert_eq!(board.get_cell(1, 3), 0);

    assert_eq!(board.get_cell(2, 0), 4);
    assert_eq!(board.get_cell(2, 1), 4);
    assert_eq!(board.get_cell(2, 2), 0);
    assert_eq!(board.get_cell(2, 3), 0);

    assert_eq!(board.get_cell(3, 0), 4);
    assert_eq!(board.get_cell(3, 1), 0);
    assert_eq!(board.get_cell(3, 2), 0);
    assert_eq!(board.get_cell(3, 3), 0);

    assert!(board.get_max_num() == 8);
}

#[test]
fn move_down_stacked() {
    let mut board = Board::new(4);
    board.set_cell(0, 0, 2);
    board.set_cell(0, 2, 8);
    board.set_cell(0, 3, 4);

    board.move_dir(Dir::Down);

    assert!(board.get_max_num() == 8);
    assert_eq!(board.get_cell(0, 0), 0);
    assert_eq!(board.get_cell(0, 1), 2);
    assert_eq!(board.get_cell(0, 2), 8);
    assert_eq!(board.get_cell(0, 3), 4);
    assert_eq!(board.get_cell(2, 2), 0);
}

#[test]
fn move_rigth_integration() {
    let mut board = Board::new(4);
    board.set_cell(2, 0, 2);
    board.set_cell(2, 3, 4);

    board.move_dir(Dir::Right);

    assert_eq!(board.get_cell(3, 0), 2);
    assert_eq!(board.get_cell(3, 3), 4);
    assert_eq!(board.get_cell(2, 0), 0);
    assert_eq!(board.get_cell(2, 3), 0);
}

#[test]
// 2  0
// 4  8
fn one_empty() {
    let mut board = Board::new(2);
    board.set_cell(0, 0, 2);
    board.set_cell(0, 1, 4);
    board.set_cell(1, 1, 8);

    // cell (1,0) is free!
    assert_eq!(false, board.move_dir(Dir::Left));
    assert_eq!(true, board.move_dir(Dir::Right));
}

#[test]
// 2  8
// 4  8
fn one_adjacent_down() {
    let mut board = Board::new(2);
    board.set_cell(0, 0, 2);
    board.set_cell(0, 1, 4);
    board.set_cell(1, 1, 8);
    // cell (1,0) is adjacent to (1,1) with value 8
    board.set_cell(1, 0, 8);
    assert_eq!(false, board.move_dir(Dir::Left));
    assert_eq!(true, board.move_dir(Dir::Down));
}

#[test]
// 2  8
// 4  8
fn one_adjacent_up() {
    let mut board = Board::new(2);
    board.set_cell(0, 0, 2);
    board.set_cell(0, 1, 4);
    board.set_cell(1, 1, 8);
    // cell (1,0) is adjacent to (1,1) with value 8
    board.set_cell(1, 0, 8);
    assert_eq!(false, board.move_dir(Dir::Left));
    assert_eq!(true, board.move_dir(Dir::Up));
}

#[test]
fn lost_all_different() {
    let mut board = Board::new(2);
    board.set_cell(0, 0, 2);
    board.set_cell(0, 1, 4);
    board.set_cell(1, 1, 8);
    // all the cells are different
    board.set_cell(1, 0, 16);
    assert_eq!(false, board.move_dir(Dir::Left));
    assert_eq!(false, board.move_dir(Dir::Up));
    assert_eq!(false, board.move_dir(Dir::Right));
    assert_eq!(false, board.move_dir(Dir::Down));
    assert_eq!(false, board.try_add_number());
}
