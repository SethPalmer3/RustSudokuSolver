pub mod sudoku_solver;
use std::env;
use std::fs::File;
use std::io::{self, Read};

use crate::sudoku_solver::structures::SudokuBoard;
use crate::sudoku_solver::solvers::backtace_solver;
use crate::sudoku_solver::sudoku_file_rw::parse_board;

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut content = String::new();

    // Check if file path was provided
    if args.len() != 2 {
        for _i in 0..9 {
            let _ = io::stdin().read_line(&mut content);
        }
    }else {
        let mut file = match File::open(&args[1]) {
            Ok(it) => it,
            Err(_) => panic!("Could not read file!"),
        };
        let _ = file.read_to_string(&mut content);
    }

    // Read the file
    let mut board = parse_board(content);
    println!("{}", board.stringify());
    let mut solutions: Vec<SudokuBoard> = Vec::new();
    backtace_solver(&mut board, &mut solutions);
    if solutions.len() >= 1{
        println!("Solved: ");
        for _b in solutions {
            println!("{}", _b.stringify());
        }
    }else{
        println!("No solution");
    }
}
