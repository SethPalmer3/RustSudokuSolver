use super::structures::SudokuBoard;

pub(crate) fn backtace_solver(sud_board: &mut SudokuBoard, solution_vec: &mut Vec<SudokuBoard>){
    sud_board.solve_board();
    if sud_board.is_solved(){
        solution_vec.push(sud_board.clone());
        return;
    }
    if sud_board.has_impossible_cells() || !sud_board.is_valid(){
        return;
    }
    for rr in 0..9 {
        for cc in 0..9 {
            let cell = sud_board.get_cell(rr, cc);
            if cell.is_locked(){
                continue;
            }
            for zz in cell.get_value_vec() {
                let mut board_clone = sud_board.clone();
                board_clone.set_values(vec![(rr,cc,zz)]);
                backtace_solver(&mut board_clone, solution_vec);
            }
            return;
        }
    }
}
