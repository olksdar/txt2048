//!
//! txt2048 is a library that makes easy implementation of your own 2048 game.
//!
//! # Examples
//! ```
//! use txt2048::Board;
//! let mut board = Board::new(4);
//! board.init();
//!
//! let move_fn = |dir| board.move_dir(dir);
//! let cells_info = board.get_cells();
//! let check_win_fn = || board.check_win();
//! ```
//! Or check txt2048 main for complete simple game implementation.

use rand::{thread_rng, Rng};
use std::cmp;

/// Represents a game board
///
/// Board has to be created using function [Board::new], see example:
///
/// # Examples
/// ```
/// use txt2048::Board;
///
/// let mut board = Board::new(4);
/// board.init();
/// ```
///
/// The game board is game context holder.
///
#[derive(Clone)]
pub struct Board {
    size: usize,
    cells: Vec<u32>,
    max_num: u32,
}

/// Possible directions are: Left, Right, Up and Down.
#[derive(Debug)]
pub enum Dir {
    Left,
    Right,
    Up,
    Down,
}

impl Board {
    /// Creates a new board with given size.
    pub fn new(size: usize) -> Board {
        Board {
            size,
            cells: vec![0; size * size],
            max_num: 0,
        }
    }

    /// Returns size of the board.
    pub fn size(&self) -> usize {
        self.size
    }

    /// Returns a vector with board cells.
    /// Cells are ordered from left to right, top to bottom.
    /// The returning vector is single dimensional. To get value based on x, y coordinates use
    /// [Board::get_cell] function.
    pub fn get_cells(&self) -> &Vec<u32> {
        &self.cells
    }

    /// Returns maximum number on the board.
    pub fn get_max_num(&self) -> u32 {
        self.max_num
    }

    /// Returns a cell value based on x, y coordinates.
    pub fn get_cell(&mut self, x: usize, y: usize) -> u32 {
        self.cells[self.get_index(x, y)]
    }

    /// Generates a number to be added to the board.
    /// The returning value is one of 2, 4 or 8.
    fn generate(&self) -> u32 {
        let mut rng = thread_rng();
        match rng.gen_range(0, 2) as u8 {
            0 => 2,
            1 => 4,
            _ => 8,
        }
    }

    /// Returns index based on x, y coordinates.
    fn get_index(&self, x: usize, y: usize) -> usize {
        (y * self.size + x) % self.cells.len()
    }

    /// Returns a vector with free position indexes.
    /// This is a helper function that used for figuring out where the next item can be placed.
    fn get_free_idx(&self) -> Vec<usize> {
        let mut indexes = Vec::new();

        for (i, x) in self.cells.iter().enumerate() {
            if *x == 0 {
                indexes.push(i);
            }
        }
        indexes
    }

    /// Returns free available cell.
    /// If there are no free cells returns None.
    fn get_free_cell(&mut self) -> Option<&mut u32> {
        let v = self.get_free_idx();
        if v.is_empty() {
            return None;
        }
        let mut rng = thread_rng();
        let idx = rng.gen_range(0, v.len());
        Some(&mut self.cells[v[idx]])
    }

    /// Wrapper that updates board maximum value.
    /// returns original value, passed to num.
    fn update_max(&mut self, num: u32) -> u32 {
        self.max_num = cmp::max(num, self.max_num);
        num
    }

    /// Tries to add a number to the board.
    /// Returns true if attempt successed.
    /// Returns false in case no space available on the board.
    pub fn try_add_number(&mut self) -> bool {
        let v = self.update_max(self.generate());
        match self.get_free_cell() {
            None => return false,
            Some(freecell) => *freecell = v,
        }
        return true;
    }

    /// Initializes the board.
    /// Initially board has 2 numbers on it.
    pub fn init(&mut self) {
        self.try_add_number();
        self.try_add_number();
    }
    ///
    /// fn move_any: shifts Board cells using iterator function get_idx
    /// If two adjacent cells has same value then they are changed by one cell with value equal
    /// to double of former value.
    ///
    /// @return: true in case changes were made after move i.e. not game end.
    /// In case no moves are available returns false, i.e. game end.
    ///
    /// @param get_idx: function that converts start position and index into appropriate position in the
    /// array so it acts like an iterator. For example call(start_row, i) for i: 0..3 will return start_row,
    /// start_raw + 1, start_row + 2, start_row + 3.
    /// @param init_pos: start position to iterate from
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

    /// Updates board with shifting all the numbers at direction side.
    /// Returns true if shift happened.
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

    /// Returns true if game is won.
    pub fn check_win(&self) -> bool {
        return self.max_num >= 2048;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_basics() {
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

    fn testing_cells_grid() -> &'static [u32] {
        #[rustfmt::skip]
        static ARRAY: [u32; 16] = [
        2, 0, 2, 0, 
        2, 2, 2, 2, 
        0, 4, 0, 2, 
        0, 4, 4, 0];

        &ARRAY
    }

    #[test]
    fn move_left() {
        let mut board = Board::new(4);
        board.cells = testing_cells_grid().to_vec();

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
        board.cells = testing_cells_grid().to_vec();

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
        board.cells = testing_cells_grid().to_vec();

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
        board.cells = testing_cells_grid().to_vec();

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
        board.cells = testing_cells_grid().to_vec();

        #[rustfmt::skip]
        let cells = [
            2, 0, 0, 0, 
            8, 0, 0, 0, 
            0, 0, 0, 0, 
            4, 0, 0, 0];
        board.cells = cells.to_vec();

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

        #[rustfmt::skip]
        let cells = [
            0, 0, 2, 0, 
            0, 0, 0, 0, 
            0, 0, 0, 0, 
            4, 0, 0, 0];
        board.cells = cells.to_vec();

        board.move_dir(Dir::Right);

        assert_eq!(board.get_cell(3, 0), 2);
        assert_eq!(board.get_cell(3, 3), 4);
        assert_eq!(board.get_cell(2, 0), 0);
        assert_eq!(board.get_cell(2, 3), 0);
    }

    #[test]
    fn one_empty() {
        let mut board = Board::new(2);
        #[rustfmt::skip]
        let cells = [
            2, 0, 
            4, 8];
        board.cells = cells.to_vec();

        // check that cell (1,0) is free
        assert_eq!(false, board.move_dir(Dir::Left));
        assert_eq!(true, board.move_dir(Dir::Right));
    }

    #[test]
    fn one_adjacent_down() {
        let mut board = Board::new(2);

        // cell (1,0) is adjacent to (1,1) with value 8
        #[rustfmt::skip]
        let cells = [
            2, 8, 
            4, 8];
        board.cells = cells.to_vec();

        assert_eq!(false, board.move_dir(Dir::Left));
        assert_eq!(true, board.move_dir(Dir::Down));
    }

    #[test]
    fn one_adjacent_up() {
        let mut board = Board::new(2);

        // cell (1,0) is adjacent to (1,1) with value 8
        #[rustfmt::skip]
        let cells = [
            2, 8, 
            4, 8];
        board.cells = cells.to_vec();

        assert_eq!(false, board.move_dir(Dir::Left));
        assert_eq!(true, board.move_dir(Dir::Up));
    }

    #[test]
    fn game_lost() {
        let mut board = Board::new(2);
        // no progress possible
        #[rustfmt::skip]
        let cells = [
            2, 4, 
            4, 8];
        board.cells = cells.to_vec();

        assert_eq!(false, board.move_dir(Dir::Left));
        assert_eq!(false, board.move_dir(Dir::Up));
        assert_eq!(false, board.move_dir(Dir::Right));
        assert_eq!(false, board.move_dir(Dir::Down));
        assert_eq!(false, board.try_add_number());
    }
}
