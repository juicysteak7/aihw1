use std::fs::File;
use std::io::{self, BufRead};

use std::collections::HashSet;
use std::collections::VecDeque;

// Define a structure to represent the puzzle board
#[derive(Eq, Hash, PartialEq, Clone)]
struct Board {
    rows: usize,
    cols: usize,
    board: Vec<Vec<char>>,
}

impl Board {
    fn new(board: Vec<Vec<char>>, rows: usize, cols: usize) -> Self {
        Board {
            board,
            rows,
            cols,
        }
    }
}

fn main() {
    let mut read_board:Vec<Vec<char>> = Vec::new();
    match read_bug_rush_board("bugrush.txt") {
        Ok(result) => {
            read_board = result;
        }
        Err(error) => {
            println!("{}",error);
        }
    }
    let rows = read_board.len();
    let cols = read_board[0].len();
    let mut board:Board = Board::new(read_board, rows, cols);
    // if !board.is_solvable() {
    //     println!("Unsat");
    //     std::process::exit(1);
    // }
    // match board.solve() {
    //     Some(result) => {
    //         println!("Result: {}", result);
    //     }
    //     None => {
    //         println!("None");
    //     }
    // }
    print_board(&board.board);
}

fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        for character in row {
            print!("{}",character);
        }
        println!("");
    }
}

fn read_bug_rush_board(filename: &str) -> io::Result<Vec<Vec<char>>> {
    let file = File::open(filename)?;
    let reader = io::BufReader::new(file);

    // Initialize the 2D vector to store the board
    let mut board: Vec<Vec<char>> = Vec::new();

    for line in reader.lines() {
        let line = line?;
        let row: Vec<char> = line.chars().collect();
        board.push(row);
    }

    Ok(board)
}
