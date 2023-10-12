use std::fs::File;
use std::io::{self,BufRead};
// use rand::Rng;

use std::collections:: {HashMap, HashSet, VecDeque, BinaryHeap};

// Define a structure to represent the puzzle board
#[derive(Eq, Hash, PartialEq, Ord, PartialOrd, Clone)]
struct Board {
    rows: usize,
    cols: usize,
    board: Vec<Vec<char>>,
}

impl Board {
    fn new(board: Vec<Vec<char>>, rows: usize, cols: usize) -> Board {
        Board {
            board,
            rows,
            cols,
        }
    }
}

fn main() {
    // Code to read a bug rush board from a file
    let mut read_board:Vec<Vec<char>> = Vec::new();
    match read_bug_rush_board("bugrush.bugs") {
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


    // Code to generate a random game board
    // let n = 6;
    // let mut board:Board = Board::new(generate_random_bug_rush(n),n,n);
    // while !is_solvable(&board.board, false) {
    //     board.board = generate_random_bug_rush(n);
    // }

    println!("~~~~~~~~~~~ Initial board state ~~~~~~~~~~~");
    print_board(&board.board);
    println!("~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~");
    let solution = solve_bug_rush_astar(&board);
    let mut steps:usize = 0;
    let mut bfs = false;
    let mut astar_steps = 0;
    match solution {
        Some(result) => {
            for step in result {
                println!("~~~~~~~~~~ Step {} ~~~~~~~~~~", steps);
                steps += 1;
                print_board(&step.board);
            }
            println!("Solved using A*. Number of steps taken: {}", steps-1);
            astar_steps = steps - 1;
        }
        None => {
            steps = 0;
            let retry = solve_bug_rush_bfs(&board);
            match retry {
                Some(new_result) => {
                    for step in new_result {
                        println!("~~~~~~~~~~ Step {} ~~~~~~~~~~", steps);
                        steps += 1;
                        print_board(&step.board);
                    }
                    let bfs_steps = steps - 1;
                    println!("Solved using BFS. Number of steps taken: {}", steps-1);
                    println!("A* steps: {}, BFS steps: {}", astar_steps, bfs_steps);
                    bfs = true;
                }
                None => {
                    println!("unsat");
                    bfs = true;
                }
            }
        }
    }
    if !bfs {
        steps = 0;
        let solution = solve_bug_rush_bfs(&board);
            match solution {
                Some(new_result) => {
                    for step in new_result {
                        println!("~~~~~~~~~~ Step {} ~~~~~~~~~~", steps);
                        steps += 1;
                        print_board(&step.board);
                    }
                    let bfs_steps = steps - 1;
                    println!("Solved using BFS. Number of steps taken: {}", steps-1);
                    println!("A* steps: {}, BFS steps: {}", astar_steps, bfs_steps);
                }
                None => {
                    println!("unsat");
                }
            }
    }
}

/// A* implementation uses basic heuristics to find solution, not gaurenteed the shortest solution
fn solve_bug_rush_astar(initial_state: &Board) -> Option<Vec<Board>> {
    let mut open_set: BinaryHeap<Board> = BinaryHeap::new();
    let mut came_from: HashMap<Board, Board> = HashMap::new();
    let mut g_score: HashMap<Board, u32> = HashMap::new();
    let mut f_score: HashMap<Board, u32> = HashMap::new();
    let mut visited: HashSet<Board> = HashSet::new();
    
    open_set.push(initial_state.clone());
    g_score.insert(initial_state.clone(), 0);
    let initial_f_score = heuristic(initial_state);
    f_score.insert(initial_state.clone(), initial_f_score);

    while let Some(current) = open_set.pop() {
        visited.insert(current.clone());

        // If current is the goal state, reconstruct the path and return it
        if is_solved(&current) {
            return build_solution_path(&came_from, &current);
        }

        // Iterate through neighbors and update g_scores
        for neighbor in neighbors(&current) {
            if visited.contains(&neighbor) {
                continue; // Skip visited states
            }

            if g_score.contains_key(&current){
                let tentative_g_score: u32 = g_score[&current] + 1;
                let temp_f_score = tentative_g_score + heuristic(&neighbor);

                if (f_score.contains_key(&current) && f_score[&current] >= temp_f_score) || !g_score.contains_key(&neighbor) {
                    // print_board(&neighbor.board);
                    // println!("{},{}",f_score[&current], temp_f_score);
                    if !came_from.contains_key(&neighbor) {
                        came_from.insert(neighbor.clone(), current.clone());
                        visited.insert(neighbor.clone());
                        g_score.insert(neighbor.clone(), tentative_g_score);
                        f_score.insert(neighbor.clone(), temp_f_score);

                        // Explicitly push the neighbor into the BinaryHeap with its F score
                        open_set.push(neighbor.clone());
                    }

                }
            }
        }
    }

    None  // No path found
}

/// Heuristic root calls different heuristics seperately to make changing the weights easy
fn heuristic(board: &Board) -> u32 {
    h3_heuristic(board) + h1_heuristic(board)
}

/// Check distance from solution
fn h1_heuristic(board: &Board) -> u32 {
    for row in 0..board.board.len() {
        for col in 0..board.board[0].len() {
            // Goal car found
            if board.board[row][col] == '>' {
                return (board.board[0].len() - col) as u32;
            }
        }
    }
    0
}


/// Check how many cars are blocking the goal car
fn h3_heuristic(board: &Board) -> u32 {
    let mut target_row = 0; // Row of the target vehicle
    let mut target_col = 0; // Column of the target vehicle
    'outer: for row in 0..board.board.len() {
        for col in 0..board.board[0].len() {
            // Goal car found
            if board.board[row][col] == '>' {
                target_col = col;
                target_row = row;
                break 'outer;

            }
        }
    }

    // Initialize counters for blocking vehicles
    let mut blocking_cars = 0;

    // Check if there are any vehicles blocking the path to the exit
    for row in 0..board.rows {
        for col in 0..board.cols {
            if board.board[row][col] != ' ' && row == target_row && col > target_col{
                blocking_cars += 1;
            }
        }
    }

    // Return the sum of both types of blocking vehicles
    blocking_cars as u32
}

/// Gets all valid moves "neighbors"
fn neighbors(board: &Board) -> Vec<Board> {
    // neighboring_states
    let mut neighboring_states = Vec::new();
    let empty_space = ' ';

    for i in 0..board.rows {
        for j in 0..board.cols {
            let current_char = board.board[i][j];

            if current_char != empty_space {
                // Handle moving the current car in all allowed directions
                let mut new_board;

                if current_char == '>' {
                    // Goal car - move right if there's empty space to the right
                    if j < board.cols - 1 && board.board[i][j + 1] == empty_space {
                        new_board = board.clone();
                        new_board.board[i][j] = empty_space;
                        new_board.board[i][j + 1] = current_char;
                        neighboring_states.push(new_board);
                    }
                    // Goal car - move left if there is an empty space
                    if j > 0 && board.board[i][j - 1] == empty_space {
                        new_board = board.clone();
                        new_board.board[i][j] = empty_space;
                        new_board.board[i][j - 1] = current_char;
                        neighboring_states.push(new_board);
                    }
                } else if current_char == '|' {
                    // Vertical car - move up or down if there's empty space in that direction
                    if i < board.rows - 1 && board.board[i + 1][j] == empty_space {
                        new_board = board.clone();
                        new_board.board[i][j] = empty_space;
                        new_board.board[i + 1][j] = current_char;
                        neighboring_states.push(new_board);
                    }
                    if i > 0 && board.board[i - 1][j] == empty_space {
                        new_board = board.clone();
                        new_board.board[i][j] = empty_space;
                        new_board.board[i - 1][j] = current_char;
                        neighboring_states.push(new_board);
                    }
                } else if current_char == '-' {
                    // Horizontal car - move left or right if there's empty space in that direction
                    if j < board.cols - 1 && board.board[i][j + 1] == empty_space {
                        new_board = board.clone();
                        new_board.board[i][j] = empty_space;
                        new_board.board[i][j + 1] = current_char;
                        neighboring_states.push(new_board);
                    }
                    if j > 0 && board.board[i][j - 1] == empty_space {
                        new_board = board.clone();
                        new_board.board[i][j] = empty_space;
                        new_board.board[i][j - 1] = current_char;
                        neighboring_states.push(new_board);
                    }
                }
            }
        }
    }

    neighboring_states
}

// /// Generates a random n by n bug rush board (doesn't gaurantee solvability)
// fn generate_random_bug_rush(n: usize) -> Vec<Vec<char>> {
//     let mut rng = rand::thread_rng();
//     let mut board: Vec<Vec<char>> = vec![vec![' '; n]; n];
//     let empty_space = ' ';
//     let horizontal_car = '-';
//     let vertical_car = '|';
//     let goal_car = '>';

//     // Fill the rest of the board with random cars
//     for row in 0..n {
//         for col in 0..n {
//             let car_type = if rng.gen_bool(0.5) { horizontal_car } else { vertical_car };
//             board[row][col] = car_type;
//         }
//     }

//     // Make some of the cars empty spaces
//     for _ in 0..(n * n / 2) {
//         let row = rng.gen_range(0..n);
//         let col = rng.gen_range(0..n);
//         board[row][col] = empty_space;
//     }

//     // Place the goal car ('>') in the starting position
//     board[n - 1][0] = goal_car;

//     board
// }

fn print_board(board: &Vec<Vec<char>>) {
    for row in board {
        for character in row {
            print!("{}",character);
        }
        println!();
    }
}

/// Reads a bug rush board from file and returns it
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

// /// Checks a board for solvability
// fn is_solvable(board: &Vec<Vec<char>>, ignore_top_row: bool) -> bool {
//     // Make sure goal care isn't blocked by horozontal cars
//     let mut goal_car_found = false;
//     for row in board {
//         for col in row {
//             if *col == '>' {
//                 goal_car_found = true;
//             } else {
//                 if goal_car_found && *col == '-'{
//                     return false;
//                 }
//             }
//         }
//         if goal_car_found {
//             break;
//         }
//     }
//     let mut flattened_board: Vec<char> = board.iter().flatten().cloned().collect();
//     if ignore_top_row {
//         flattened_board = board.iter().skip(1).flatten().cloned().collect();
//     }

//     // Count the number of inversions
//     let mut inversions = 0;
//     for i in 0..flattened_board.len() {
//         for j in i + 1..flattened_board.len() {
//             if flattened_board[i] != ' ' && flattened_board[j] != ' ' && flattened_board[i] > flattened_board[j] {
//                 inversions += 1;
//             }
//         }
//     }

//     // If the number of inversions is even, the puzzle is solvable
//     inversions % 2 == 0
// }

/// BFS solution - gaurentees shortest path, doesn't gaurentee any effeciency
fn solve_bug_rush_bfs(initial_state: &Board) -> Option<Vec<Board>> {
    let mut queue: VecDeque<Board> = VecDeque::new();
    let mut visited: HashSet<Board> = HashSet::new();
    let mut parent: HashMap<Board, Board> = HashMap::new();

    queue.push_back(initial_state.clone());
    visited.insert(initial_state.clone());

    while let Some(current_state) = queue.pop_front() {
        // print_board(&current_state.board);
        if is_solved(&current_state) {
            // Build and return the sequence of states representing the solution path
            return build_solution_path(&parent, &current_state);
        }

        for next_state in neighbors(&current_state) {
            if !visited.contains(&next_state) && !parent.contains_key(&next_state) {
                visited.insert(next_state.clone());
                parent.insert(next_state.clone(), current_state.clone());
                queue.push_back(next_state);
            }
        }
    }

    None // No solution found (make sure board is solvable before running this)
}

/// Backtracks the steps made to find the solution so it can be printed nicely
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
