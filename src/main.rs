fn main() {
    let mut board:Vec<Vec<char>> = vec![vec!['-','-'],vec![' ','-'],vec!['>','|']];
    //println!("{:?}",board);
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

fn step(board: &Vec<Vec<char>>) -> Result<Vec<Vec<char>>,&'static str> {
    return Err("Err");
}
