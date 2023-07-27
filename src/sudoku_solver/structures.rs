use crate::sudoku_solver::helper_functions::board_to_block;
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
    pub(crate) fn no_possible_value(&self) -> bool {
        return self.possible_values == 0;
    }
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
    pub(crate) fn get_value(&self) -> i16 {
        if self.locked{
            return self.possible_values;
        }else{
            return 0;
        }
    }
    pub(crate) fn remove_possible_values(&mut self, values: i16){
        if !self.locked{
            let mask = ALL_POSSIBLE ^ values;
            self.possible_values &= mask;
            // println!("{},{} could be {:?}", self.row, self.col, self.get_value_vec().iter().map(|v| encode_to_decimal(*v)).collect::<Vec<i32>>());
        }
    }
    pub(crate) fn apply_lock(&mut self) -> bool {
        if self.get_value_vec().len() == 1{
            self.locked = true;
            return true;
        }
        return false;
    }
    pub(crate) fn change_possible_values(&mut self, values: i16) {
        self.possible_values = values;
    }
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
    pub(crate) fn is_locked(&self) -> bool {
        self.locked
    }
}

impl SudokuBoard {
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
    pub(crate) fn apply_to_cells<F>(&mut self, mut ff: F) where F: FnMut(&mut SudokuCell) -> () {
        for rr in 0..9 {
            for cc in 0..9 {
                let cell = &mut self.grid[rr][cc];
                ff(cell);
            }
        }
    }
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
    pub(crate) fn set_values(&mut self, values: Vec<(usize, usize, i16)>) -> bool {
        for (r,c,v) in values{
            self.grid[r][c].possible_values = v;
        }
        self.apply_lock_board();
        return true;
    }
    pub(crate) fn apply_lock_board(&mut self) -> bool {
        let mut new_locks = false;
        self.apply_to_cells(|cell| new_locks |= cell.apply_lock());
        return new_locks;
    }
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
    pub(crate) fn solve_row(&mut self, r: usize) {
        let known_vals = self.known_rows(r);
        for cell in &mut self.grid[r]{
            cell.remove_possible_values(known_vals);
        }
    }
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
    pub(crate) fn solve_col(&mut self, c: usize) {
        let known_vals = self.known_cols(c);
        for rr in 0..9{
            let cell = &mut self.grid[rr][c];
            cell.remove_possible_values(known_vals);
        }
    }
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
    pub(crate) fn has_impossible_cells(&mut self) -> bool {
        let mut no_possible = false;
        self.apply_to_cells(|cell| no_possible |= cell.no_possible_value());
        return no_possible;
    }

    pub(crate) fn is_solved(&mut self) -> bool {
        let mut is_solved = true;
        self.apply_to_cells(|cell|{
            is_solved &= cell.locked;
        });
        is_solved &= !self.has_impossible_cells();
        is_solved &= self.is_valid();
        return is_solved;
    }

    pub(crate) fn is_valid(&self) -> bool {
        let mut valid = true;
        for zz in 0..9 {
            valid &= self.is_valid_row(zz);
            valid &= self.is_valid_col(zz);
            valid &= self.is_valid_blk(zz);
        }
        return valid;
    }

    pub(crate) fn is_valid_spot(&self, rr: usize, cc: usize) -> bool {
        let mut valid = true;
        valid &= self.is_valid_row(rr);
        valid &= self.is_valid_col(cc);
        valid &= self.is_valid_blk(board_to_block(rr, cc));
        return valid;
    }

    pub(crate) fn get_cell(&mut self, rr: usize, cc: usize) -> &mut SudokuCell {
        &mut self.grid[rr][cc]
    }
}
