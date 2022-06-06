use std::io;
use std::io::Write;

const NUM_COLS: usize = 7;
const NUM_ROWS: usize = 6;

fn main() {
    let mut board = vec![vec![0; NUM_COLS]; NUM_ROWS];
    run_game(&mut board);
}

fn display_board(board: &Vec<Vec<usize>>) {
    let mut row: usize = 0;
    let mut col: usize = 0;

    while row < NUM_ROWS {
        col = 0;
        while col < NUM_COLS {
            print!("{} ", board[row][col]);
            io::stdout().flush().unwrap();
            col += 1;
        }
        println!();
        row += 1;
    }
}

fn is_board_full(board: &Vec<Vec<usize>>) -> bool {
    let mut col: usize = 0;

    while col < NUM_COLS {
        if board[0][col] == 0 {
            return false;
        }
        col += 1;
    }
    true
}

fn is_victory(board: &Vec<Vec<usize>>, player_num: usize) -> bool {
    let mut row: usize = 0;
    let mut col: usize = 0;

    // horizontal
    while row < NUM_ROWS {
        col = 0;
        while col < NUM_COLS - 3 {
            if board[row][col + 0] == player_num
                && board[row][col + 1] == player_num
                && board[row][col + 2] == player_num
                && board[row][col + 3] == player_num
            {
                println!("Horizontal connect 4 at row {}", row);
                return true;
            }
            col += 1;
        }
        row += 1;
    }

    // vertical
    col = 0;
    while col < NUM_COLS {
        row = 0;
        while row < NUM_ROWS - 3 {
            if board[row + 0][col] == player_num
                && board[row + 1][col] == player_num
                && board[row + 2][col] == player_num
                && board[row + 3][col] == player_num
            {
                println!("Vertical connect 4 at col {}", col);
                return true;
            }
            row += 1;
        }
        col += 1;
    }

    row = 0;
    // ascending diagonal
    while row < NUM_ROWS {
        col = 0;
        while col < NUM_COLS - 3 {
            if board[row + 0][col + 0] == player_num
                && board[row - 1][col + 1] == player_num
                && board[row - 2][col + 2] == player_num
                && board[row - 3][col + 3] == player_num
            {
                println!("Diagonal connect 4 at row {}", row);
                return true;
            }
            col += 1;
        }
        row += 1;
    }

    row = 0;
    // descending diagonal
    while row < NUM_ROWS {
        col = 0;
        while col < NUM_COLS {
            if board[row + 0][col + 0] == player_num
                && board[row - 1][col - 1] == player_num
                && board[row - 2][col - 2] == player_num
                && board[row - 3][col - 3] == player_num
            {
                println!("Diagonal connect 4 at row {}", row);
                return true;
            }
            col += 1;
        }
        row += 1;
    }

    false
}

fn drop_piece(board: &mut Vec<Vec<usize>>, col: usize, player_num: usize) -> bool {
    let mut row: usize = NUM_ROWS - 1;

    if board[0][col] != 0 {
        return false;
    }

    while row >= 0 {
        if board[row][col] == 0 {
            board[row][col] = player_num;
            break;
        }
        row -= 1;
    }

    return true;
}

fn run_game(board: &mut Vec<Vec<usize>>) {
    let mut current_player = 1;

    'game: loop {
        display_board(&board);

        if is_victory(&board, 1) {
            println!("PLAYER 1 Wins!!");
            break 'game;
        }

        if is_victory(&board, 2) {
            println!("PLAYER 2 Wins!!");
            break 'game;
        }

        if is_board_full(&board) {
            println!("Board is full. Stalemate.");
            break 'game;
        }

        println!("Board is not full. Play a round.");
        println!("Player {} enter a column to drop piece.", current_player);

        'drop: loop {
            let mut buffer = String::new();
            io::stdin()
                .read_line(&mut buffer)
                .expect("Error getting column");
            let input = buffer.trim();

            if drop_piece(board, input.parse::<usize>().unwrap(), current_player) {
                break 'drop;
            }
            println!("Slot {} is full. Try again.", input);
        }

        if current_player == 1 {
            current_player = 2;
        } else if current_player == 2 {
            current_player = 1;
        }
    }
}
