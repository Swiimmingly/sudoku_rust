use rand::prelude::*;

const SIZE: usize = 9;

fn main() {
    let mut board = [[0; SIZE]; SIZE];
    // println!("{:?}",board);

    fill_board(&mut board,2);

    //make sure board ahs a solution
    let mut solved_board = board.clone();
    solve_sudoku(&mut solved_board);

    while contains_zero(&solved_board){
        let mut board = [[0; SIZE]; SIZE];
        fill_board(&mut board,2);
        solve_sudoku(&mut solved_board);
    }

    print_board(&board);
    solve_sudoku(&mut board);
    print_board(&board);


}

fn contains_zero(board: &[[u8; SIZE]; SIZE]) -> bool {
    board.iter().any(|row| row.iter().any(|&element| element == 0))
}

fn fill_board(board: &mut [[u8; SIZE]; SIZE], difficulty: u8) {
    let mut rng = rand::thread_rng();
    for i in 0..difficulty{
        for num in 1..=SIZE as u8 {
            let mut row = 0;
            let mut col = 0;

            loop {
                row = rng.gen_range(0..SIZE);
                col = rng.gen_range(0..SIZE);

                if is_valid_move(board, row, col, num) {
                    board[row][col] = num;
                    break;
                }
            }
        }
    }
}

fn is_valid_move(board: &[[u8; SIZE]; SIZE], row: usize, col: usize, num: u8) -> bool {
    for i in 0..SIZE {
        if board[row][i] == num || board[i][col] == num {
            return false;
        }
    }

    let start_row = 3 * (row / 3);
    let start_col = 3 * (col / 3);

    for i in 0..3 {
        for j in 0..3 {
            if board[start_row + i][start_col + j] == num {
                return false;
            }
        }
    }

    true
}

fn print_board(board: &[[u8; SIZE]; SIZE]) {
    for row in board.iter() {
        for &cell in row.iter() {
            print!("{} ", cell);
        }
        println!();
    }
    println!("=======================================================")
}


fn solve_sudoku(board: &mut [[u8; SIZE]; SIZE]) -> bool {
    for row in 0..SIZE {
        for col in 0..SIZE {
            if board[row][col] == 0 {
                for num in 1..=SIZE as u8 {
                    if is_valid_move(board, row, col, num) {
                        board[row][col] = num;

                        if solve_sudoku(board) {
                            return true;
                        }

                        board[row][col] = 0; // Backtrack
                    }
                }
                // print_board(&board);
                return false; // No valid num
            }
        }
    }

    true // solved
}

