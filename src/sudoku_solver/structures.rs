use crate::sudoku_solver::helper_functions::block_to_board;
use super::helper_functions::encode_to_decimal;

const ALL_POSSIBLE: i16 = 0b111111111;

#[derive(Clone)]
pub struct SudokuBoard{
    grid: Vec<Vec<SudokuCell>>,
}

#[derive(Clone)]
pub struct SudokuCell{
    possible_values: i16,
    locked: bool,
}

impl SudokuCell {
    pub(crate) fn new() -> SudokuCell{
        let new_cell = SudokuCell{possible_values: ALL_POSSIBLE, locked: false};
        return new_cell;
    }
    // checks if there is no possible solutions for this cell
    pub(crate) fn no_possible_value(&self) -> bool {
        return self.possible_values == 0;
    }
    // get a string for this cell
    pub(crate) fn stringify(&self) -> String {
        if self.no_possible_value() {
            return String::from("0");
        }
        if self.locked{
            return encode_to_decimal(self.possible_values).to_string();
        }else{
            return String::from(".");
        }
    }
    // get the value if the cell has a known value
    pub(crate) fn get_value(&self) -> i16 {
        if self.locked{
            return self.possible_values;
        }else{
            return 0;
        }
    }
    // removes possible values from cell possiblities
    pub(crate) fn remove_possible_values(&mut self, values: i16){
        if !self.locked{
            let mask = ALL_POSSIBLE ^ values;
            self.possible_values &= mask;
        }
    }
    // tries to apply lock to a cell returns if lock succeeded 
    pub(crate) fn apply_lock(&mut self) -> bool {
        if self.get_value_vec().len() == 1{
            self.locked = true;
            return true;
        }
        return false;
    }
    // get all possible values from an unkown cell
    pub(crate) fn get_value_vec(&self) -> Vec<i16> {
        if self.locked{
            return vec![];
        }
        let mut check = 0;
        let mut vec_values: Vec<i16> = Vec::new();
        while check < 9{
            if self.possible_values & 1<<check != 0 {
                vec_values.push(1<<check);
            }
            check += 1;
        }
        return vec_values;
    }
    // returns if cell is locked
    pub(crate) fn is_locked(&self) -> bool {
        self.locked
    }
}

impl SudokuBoard {
    // return a new empty board
    pub(crate) fn new_empty_board() -> SudokuBoard{
        let mut board: Vec<Vec<SudokuCell>> = Vec::new();
        for _rr in 0..9{
            let mut row: Vec<SudokuCell> = Vec::new();
            for _cc in 0..9{
                row.push(SudokuCell::new());
            }
            board.push(row);
        }
        return SudokuBoard{grid: board};
    }
    // apply a function to all cells in the board
    // ** This is probably not necessary **
    pub(crate) fn apply_to_cells<F>(&mut self, mut ff: F) where F: FnMut(&mut SudokuCell) -> () {
        for rr in 0..9 {
            for cc in 0..9 {
                let cell = &mut self.grid[rr][cc];
                ff(cell);
            }
        }
    }
    // give the string of the board
    pub(crate) fn stringify(&self) -> String {
        let mut s = String::from("");
        for rr in 0..9 {
            for cc in 0..9 {
                s.push_str(self.grid[rr][cc].stringify().as_str());
            }
            s.push('\n');
        }
        return s;
    }
    // given a vector of tuples set values on the board given this format (row#, col#, value)
    pub(crate) fn set_values(&mut self, values: Vec<(usize, usize, i16)>) -> bool {
        for (r,c,v) in values{
            self.grid[r][c].possible_values = v;
        }
        self.apply_lock_board();
        return true;
    }
    // tries to lock all cells returns if any cell changed lock state
    pub(crate) fn apply_lock_board(&mut self) -> bool {
        let mut new_locks = false;
        self.apply_to_cells(|cell| new_locks |= cell.apply_lock());
        return new_locks;
    }
    // checks if a row is a valid row (i.e. no two cells have the same known value)
    pub(crate) fn is_valid_row(&self, r: usize) -> bool {
        let mut known_vals: i16 = 0;
        for cc in 0..9{
            let cell = &self.grid[r][cc];
            if cell.locked{
                if cell.get_value() & known_vals == 0 {
                    known_vals |= cell.get_value();
                }else{
                    return false;
                }
            }
        }
        return true;
    }
    // gets all the known values in a row
    pub(crate) fn known_rows(&self, r: usize) -> i16{
        let mut known_vals: i16 = 0;
        for cc in 0..9{
            let cell = &self.grid[r][cc];
            if cell.locked{
                known_vals |= cell.get_value();
            }
        }
        return known_vals;
    }
    // removes all impossible values from unkown cells
    pub(crate) fn solve_row(&mut self, r: usize) {
        let known_vals = self.known_rows(r);
        for cell in &mut self.grid[r]{
            cell.remove_possible_values(known_vals);
        }
    }
    // checks if column is valid
    pub(crate) fn is_valid_col(&self, c: usize) -> bool {
        let mut known_vals: i16 = 0;
        for rr in 0..9{
            let cell = &self.grid[rr][c];
            if cell.locked{
                if cell.get_value() & known_vals == 0 {
                    known_vals |= cell.get_value();
                }else{
                    return false;
                }
            }
        }
        return true;
    }
    // get all known values from column
    pub(crate) fn known_cols(&self, c: usize) -> i16 {
        let mut known_vals: i16 = 0;
        for rr in 0..9{
            let cell = &self.grid[rr][c];
            if cell.locked{
                known_vals |= cell.get_value();
            }
        }
        return known_vals;
    }
    // removes all impossible values from unkown cells
    pub(crate) fn solve_col(&mut self, c: usize) {
        let known_vals = self.known_cols(c);
        for rr in 0..9{
            let cell = &mut self.grid[rr][c];
            cell.remove_possible_values(known_vals);
        }
    }
    //   |   |   
    // 0 | 1 | 2 
    //---+---+---
    //   |   |   
    // 3 | 4 | 5 
    //---+---+---
    //   |   |   
    // 6 | 7 | 8 
    //
    // checks if a block is valid (b is index as indicated above)
    pub(crate) fn is_valid_blk(&self, b: usize) -> bool {
        let mut known_vals: i16 = 0;
        let (row, col) = block_to_board(b);
        for rr in row..row+3{
            for cc in col..col+3{
                let cell = &self.grid[rr][cc];
                if cell.locked{
                    if cell.get_value() & known_vals == 0{
                        known_vals |= cell.get_value();
                    }else {
                        return false;
                    }
                }
            }
        }
        return true;
    }
    // gets all known values from a block
    pub(crate) fn known_blks(&self, b: usize) -> i16 {
        let mut known_vals: i16 = 0;
        let (row, col) = block_to_board(b);
        for rr in row..row+3{
            for cc in col..col+3{
                let cell = &self.grid[rr][cc];
                if cell.locked{
                    known_vals |= cell.get_value();
                }
            }
        }
        return known_vals;
    }
    // remove all impossible values from unknown cells
    pub(crate) fn solve_blk(&mut self, b: usize) {
        let known_vals: i16 = self.known_blks(b);
        let (row, col) = block_to_board(b);
        for rr in row..row+3{
            for cc in col..col+3{
                let cell = &mut self.grid[rr][cc];
                cell.remove_possible_values(known_vals);
            }
        }
    }
    // solve entire board
    pub(crate) fn solve_board(&mut self){ // return true if board has been solved in a valid state
        loop {
            for zz in 0..9 {
                self.solve_row(zz);
                self.solve_col(zz);
                self.solve_blk(zz);
            }
            if !self.apply_lock_board() {break;}
        }
    }
    // checks if any cells have no possible values
    pub(crate) fn has_impossible_cells(&mut self) -> bool {
        let mut no_possible = false;
        self.apply_to_cells(|cell| no_possible |= cell.no_possible_value());
        return no_possible;
    }
    // checks if the board is in a solved state (has a lot of redundent checks)
    pub(crate) fn is_solved(&mut self) -> bool {
        let mut is_solved = true;
        self.apply_to_cells(|cell|{
            is_solved &= cell.locked;
        });
        is_solved &= !self.has_impossible_cells();
        is_solved &= self.is_valid();
        return is_solved;
    }
    // checks if board in a valid state (no conflicting known cells in same row, column, or block)
    pub(crate) fn is_valid(&self) -> bool {
        let mut valid = true;
        for zz in 0..9 {
            valid &= self.is_valid_row(zz);
            valid &= self.is_valid_col(zz);
            valid &= self.is_valid_blk(zz);
        }
        return valid;
    }
    // get a cell from the grid
    pub(crate) fn get_cell(&mut self, rr: usize, cc: usize) -> &mut SudokuCell {
        &mut self.grid[rr][cc]
    }
}
