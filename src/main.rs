use std::fs::File;
use std::io::{self, BufRead};

use std::collections:: {HashMap, HashSet, VecDeque};

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
    let board:Board = Board::new(read_board, rows, cols);
    if !is_solvable(&board) {
        println!("Unsat");
    }else{
        let solution = solve_bug_rush(&board);
        let mut steps:usize = 0;
        match solution {
            Some(result) => {
                for step in result {
                    steps += 1;
                    print_board(&step.board)
                }
                println!("Number of steps taken: {}", steps-1);
            }
            None => {
                println!("No solution found.");
            }
        }
    }
    // print_board(&board.board);
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

fn is_solvable(board: &Board) -> bool {
    let flattened_board: Vec<char> = board.board.iter().flatten().cloned().collect();

    // Count the number of inversions
    let mut inversions = 0;
    for i in 0..flattened_board.len() {
        for j in i + 1..flattened_board.len() {
            if flattened_board[i] != ' ' && flattened_board[j] != ' ' && flattened_board[i] > flattened_board[j] {
                inversions += 1;
            }
        }
    }

    // If the number of inversions is even, the puzzle is solvable
    inversions % 2 == 0
}

fn solve_bug_rush(initial_state: &Board) -> Option<Vec<Board>> {
    let mut queue: VecDeque<Board> = VecDeque::new();
    let mut visited: HashSet<Board> = HashSet::new();
    let mut parent: HashMap<Board, Board> = HashMap::new();

    queue.push_back(initial_state.clone());
    visited.insert(initial_state.clone());

    while let Some(current_state) = queue.pop_front() {
        //print_board(&current_state.board);
        if is_solved(&current_state) {
            // Build and return the sequence of states representing the solution path
            return build_solution_path(&parent, &current_state);
        }

        for next_state in get_possible_moves(&current_state) {
            if !visited.contains(&next_state.0) {
                visited.insert(next_state.0.clone());
                parent.insert(next_state.0.clone(), current_state.clone());
                queue.push_back(next_state.0);
            }
        }
    }

    None // No solution found
}

fn build_solution_path(
    parent: &HashMap<Board, Board>,
    goal_state: &Board,
) -> Option<Vec<Board>> {
    let mut solution_path = Vec::new();
    let mut current_state = goal_state.clone();

    while let Some(prev_state) = parent.get(&current_state) {
        solution_path.push(current_state.clone());
        current_state = prev_state.clone();
    }

    solution_path.push(current_state);

    solution_path.reverse();
    Some(solution_path)
}

fn is_solved(board: &Board) -> bool {
    let last_column = board.cols - 1;
    for i in 0..board.rows {
        if board.board[i][last_column] == '>' {
            return true; // The goal car is in the rightmost position in any row
        }
    }
    false
}

fn get_possible_moves(board: &Board) -> Vec<(Board, String)> {
    let mut possible_moves: Vec<(Board, String)> = Vec::new();
    let empty_space = ' ';
    let goal_car = '>';
    
    // Find the positions of the goal car ('>') and other cars
    let mut cars_positions = Vec::new();
    for i in 0..board.rows {
        for j in 0..board.cols {
            let current_char = board.board[i][j];
            if current_char != empty_space {
                cars_positions.push((i, j, current_char));
            }
        }
    }

    for &(i, j, current_char) in &cars_positions {
        if current_char == goal_car {
            // Handle the movement of the goal car ('>') separately
            // Ensure it only moves right if there's an empty space in front
            if j < board.cols - 1 && board.board[i][j + 1] == empty_space {
                let mut new_board = board.clone();
                new_board.board[i][j] = empty_space;
                new_board.board[i][j + 1] = goal_car;
                possible_moves.push((new_board, "Move goal car right".to_string()));
            }
        } else if current_char == '-' {
            // Handle horizontal cars ('-')
            // Ensure they move horizontally (left and right)
            if j < board.cols - 1 && board.board[i][j + 1] == empty_space {
                let mut new_board = board.clone();
                new_board.board[i][j] = empty_space;
                new_board.board[i][j + 1] = current_char;
                possible_moves.push((new_board, format!("Move {} right", current_char)));
            }
            if j > 0 && board.board[i][j - 1] == empty_space {
                let mut new_board = board.clone();
                new_board.board[i][j] = empty_space;
                new_board.board[i][j - 1] = current_char;
                possible_moves.push((new_board, format!("Move {} left", current_char)));
            }
        } else if current_char == '|' {
            // Handle vertical cars ('|')
            // Ensure they move vertically (up and down)
            if i < board.rows - 1 && board.board[i + 1][j] == empty_space {
                let mut new_board = board.clone();
                new_board.board[i][j] = empty_space;
                new_board.board[i + 1][j] = current_char;
                possible_moves.push((new_board, format!("Move {} down", current_char)));
            }
            if i > 0 && board.board[i - 1][j] == empty_space {
                let mut new_board = board.clone();
                new_board.board[i][j] = empty_space;
                new_board.board[i - 1][j] = current_char;
                possible_moves.push((new_board, format!("Move {} up", current_char)));
            }
        }
    }

    possible_moves
}
