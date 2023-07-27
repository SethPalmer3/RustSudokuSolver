
use super::structures::SudokuBoard;

/// parses a string into a SudokuBoard structure
/// use '.' for unkown cells and any number 1-9 for known cells
/// **THIS WILL NOT FIX OR WARN IF GIVEN AN INVALID BOARD STRING**
pub(crate) fn parse_board(b_string: String) -> SudokuBoard {
    let mut row_i = 0;
    let mut col_i = 0;
    let mut known_vec: Vec<(usize, usize, i16)> = Vec::new();
    for rows in b_string.split('\n'){
        for chrs in rows.chars(){
            if let Some(k) = chrs.to_digit(10){
                known_vec.push((row_i, col_i, (1<<k-1) as i16));
            }
            col_i += 1;
        }
        row_i += 1;
        col_i = 0;
    }
    let mut board: SudokuBoard = SudokuBoard::new_empty_board();
    board.set_values(known_vec);
    return board;
}
