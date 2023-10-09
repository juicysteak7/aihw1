use std::fs::File;
use std::io::{self, BufRead};

fn main() {
    let mut board:Vec<Vec<char>> = Vec::new();
    match read_bug_rush_board("bugrush.txt") {
        Ok(result) => {
            board = result;
        }
        Err(error) => {
            println!("{}",error);
        }
    }
    print_board(&board);
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
