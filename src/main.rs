use std::fs::File;
use std::io::{self, BufRead};

use std::collections::HashSet;

// Define a structure to represent the puzzle board
struct Board {
    rows: usize,
    cols: usize,
    board: Vec<Vec<char>>,
}

impl Board {
    fn new(rows: usize, cols: usize, board: Vec<Vec<char>>) -> Self {
        Board { rows, cols, board }
    }

    fn find_goal_car(&self) -> Option<(usize, usize)> {
        for (row, row_cells) in self.board.iter().enumerate() {
            for (col, &cell) in row_cells.iter().enumerate() {
                if cell == '>' {
                    return Some((row, col));
                }
            }
        }
        None // Car not found
    }
    // Check if a given position is valid and not blocked
    fn is_valid_move(&self, row: isize, col: isize) -> bool {
        if row < 0 || col < 0 || row >= self.rows as isize || col >= self.cols as isize {
            return false;
        }
        let cell = self.board[row as usize][col as usize];
        cell != '-' && cell != '|'
    }

    fn is_solvable(&self) -> bool {
        let mut inversions = 0;
        let mut empty_row = 0;
        
        // Flatten the board into a 1D vector for counting inversions
        let flat_board: Vec<char> = self.board.iter().flatten().copied().collect();

        for (i, &tile) in flat_board.iter().enumerate() {
            if tile == ' ' {
                // Track the row of the empty space
                empty_row = i / self.cols;
                continue;
            }
            
            for j in i + 1..flat_board.len() {
                if flat_board[j] != ' ' && flat_board[j] < tile {
                    inversions += 1;
                }
            }
        }

        // For odd-sized boards, solvability depends on the parity of inversions and empty row
        if self.rows % 2 == 1 {
            return inversions % 2 == 0;
        }

        // For even-sized boards, solvability depends on the parity of inversions, empty row, and row count
        if self.rows % 2 == 0 {
            return (inversions % 2 == 1 && empty_row % 2 == 0) || (inversions % 2 == 0 && empty_row % 2 == 1);
        }

        false
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
    let mut board:Board = Board::new(read_board.len(),read_board[0].len(), read_board);
    if !board.is_solvable() {
        println!("Unsat");
        std::process::exit(1);
    }
    let mut car_position = (0,0);
    match board.find_goal_car() {
        Some(result) => {
            car_position = result;
        }
        None => {

        }
    }
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

fn step(board: &Vec<Vec<char>>) -> Result<Vec<Vec<char>>,&'static str>{
    return Err("Err");
}
