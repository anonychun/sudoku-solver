fn main() {
    let mut board: [[u8; 9]; 9] = [
        [0, 7, 2, 6, 0, 1, 0, 0, 0],
        [0, 0, 5, 0, 2, 4, 0, 7, 0],
        [0, 4, 1, 7, 0, 9, 0, 2, 5],
        [0, 0, 0, 0, 9, 3, 7, 8, 0],
        [6, 0, 7, 2, 0, 8, 3, 0, 1],
        [0, 3, 8, 5, 7, 0, 0, 0, 0],
        [5, 2, 0, 9, 0, 7, 8, 1, 0],
        [0, 8, 0, 3, 1, 0, 5, 0, 0],
        [0, 0, 0, 4, 0, 5, 9, 3, 0],
    ];

    display(&board);
    println!("+ + + + + + + + + + +");
    solve(&mut board);
    println!("+ + + + + + + + + + +");
    display(&board);
}

fn display(board: &[[u8; 9]; 9]) {
    for (i_row, row) in board.iter().enumerate() {
        if i_row % 3 == 0 && i_row != 0 {
            println!("- - - - - - - - - - -")
        }
        for (i_col, col) in row.iter().enumerate() {
            if i_col % 3 == 0 && i_col != 0 {
                print!("| ")
            }
            if i_col == 8 {
                println!("{}", col)
            } else {
                print!("{} ", col)
            }
        }
    }
}

fn solve(board: &mut [[u8; 9]; 9]) -> bool {
    match find_empty(&board) {
        Some((i_row, i_col)) => {
            for num in 1..10 {
                if valid(&board, num, (i_row as usize, i_col as usize)) {
                    board[i_row as usize][i_col as usize] = num;
                    if solve(board) {
                        return true;
                    }
                    board[i_row as usize][i_col as usize] = 0;
                }
            }
        }
        None => return true,
    }

    false
}

fn find_empty(board: &[[u8; 9]; 9]) -> Option<(u8, u8)> {
    for (i_row, row) in board.iter().enumerate() {
        for (i_col, col) in row.iter().enumerate() {
            if col == &0 {
                return Some((i_row as u8, i_col as u8));
            }
        }
    }

    None
}

fn valid(board: &[[u8; 9]; 9], num: u8, position: (usize, usize)) -> bool {
    for (i_row, _) in board.iter().enumerate() {
        if board[position.0][i_row] == num && position.1 != i_row {
            return false;
        }
        if board[i_row][position.1] == num && position.0 != i_row {
            return false;
        }
    }

    let box_y = position.0 / 3;
    let box_x = position.1 / 3;
    for y in box_y * 3..box_y * 3 + 3 {
        for x in box_x * 3..box_x * 3 + 3 {
            if board[y][x] == num && y != position.0 && x != position.1 {
                return false;
            }
        }
    }

    true
}
