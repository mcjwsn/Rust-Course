use std::io;
pub fn play(){
    println!("Welcome to TicTacToe!");

    let mut board = [[' '; 3];3];
    println!("Let's start");
    println!("Insert positions like a,b\n");

    print_board(&board);
    let mut cnt:i32 = 0;
    let mut input;
    while cnt != 9{
        println!("\n");
        input = String::new();
        if cnt % 2 == 0{
            println!("Move O");
        }
        else{
            println!("Move X");
        }
        io::stdin()
            .read_line(&mut input)
            .expect("Bad read");
        if input.len() != 4 {
            println!("Bad input");
        }
        else{
            if let (Some(a), Some(b)) = (
                input.chars().nth(0).unwrap().to_digit(10),
                input.chars().nth(2).unwrap().to_digit(10),
            ) {
                let (a, b) = (a as usize, b as usize);
                if board[a][b] == ' ' {
                    board[a][b] = if cnt % 2 == 0 { 'O' } else { 'X' };
                    cnt += 1;
                    let result:bool = check_winner(&board);
                    if result{
                        print_board(&board);
                        println!("Winner is {}",board[a][b]);
                        return
                    }
                    else{
                    println!("\nCurrent state:");
                    print_board(&board);}
                } else {
                    println!("Position already taken!");
                }
            } else {
                println!("Invalid number format.");
            }
        }
    }
    println!("Game ended in a draw");
}

fn print_board(board: &[[char; 3]; 3]){
    for j in 0..2 {
            println!("{} | {} | {}", board[j][0], board[j][1], board[j][2]);
        println!("_________");
    }
    println!("{} | {} | {}", board[2][0], board[2][1], board[2][2]);
}

fn check_winner(board: &[[char; 3]; 3]) -> bool {
    for row in 0..3 {
        if board[row][0] == board[row][1] && board[row][1] == board[row][2] && board[row][0] != ' ' {
            return true;
        }
    }

    for col in 0..3 {
        if board[0][col] == board[1][col] && board[1][col] == board[2][col] && board[0][col] != ' ' {
            return true;
        }
    }

    if board[0][0] == board[1][1] && board[1][1] == board[2][2] && board[0][0] != ' ' {
        return true;
    }
    if board[0][2] == board[1][1] && board[1][1] == board[2][0] && board[0][2] != ' ' {
        return true;
    }

    false
}

#[cfg(test)]
fn make_move(board: &mut [[char; 3]; 3], row: usize, col: usize, player: char) -> Result<(), &'static str> {
    if row >= 3 || col >= 3 {
        return Err("Invalid position: out of bounds");
    }
    if board[row][col] != ' ' {
        return Err("Position already taken");
    }
    board[row][col] = player;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_empty_board() {
        let board = [[' '; 3]; 3];
        assert_eq!(check_winner(&board), false);
    }

    #[test]
    fn test_row_win() {
        // Test for first row win
        let mut board = [[' '; 3]; 3];
        for col in 0..3 {
            board[0][col] = 'X';
        }
        assert_eq!(check_winner(&board), true);

        // Test for second row win
        let mut board = [[' '; 3]; 3];
        for col in 0..3 {
            board[1][col] = 'O';
        }
        assert_eq!(check_winner(&board), true);

        // Test for third row win
        let mut board = [[' '; 3]; 3];
        for col in 0..3 {
            board[2][col] = 'X';
        }
        assert_eq!(check_winner(&board), true);
    }

    #[test]
    fn test_column_win() {
        // Test for first column win
        let mut board = [[' '; 3]; 3];
        for row in 0..3 {
            board[row][0] = 'O';
        }
        assert_eq!(check_winner(&board), true);

        // Test for second column win
        let mut board = [[' '; 3]; 3];
        for row in 0..3 {
            board[row][1] = 'X';
        }
        assert_eq!(check_winner(&board), true);

        // Test for third column win
        let mut board = [[' '; 3]; 3];
        for row in 0..3 {
            board[row][2] = 'O';
        }
        assert_eq!(check_winner(&board), true);
    }

    #[test]
    fn test_diagonal_win() {
        // Test for main diagonal win (top-left to bottom-right)
        let mut board = [[' '; 3]; 3];
        board[0][0] = 'X';
        board[1][1] = 'X';
        board[2][2] = 'X';
        assert_eq!(check_winner(&board), true);

        // Test for other diagonal win (top-right to bottom-left)
        let mut board = [[' '; 3]; 3];
        board[0][2] = 'O';
        board[1][1] = 'O';
        board[2][0] = 'O';
        assert_eq!(check_winner(&board), true);
    }

    #[test]
    fn test_no_win() {
        // Test for no win condition
        let mut board = [[' '; 3]; 3];
        board[0][0] = 'X';
        board[0][1] = 'O';
        board[0][2] = 'X';
        board[1][0] = 'X';
        board[1][1] = 'O';
        board[1][2] = 'X';
        board[2][0] = 'O';
        board[2][1] = 'X';
        board[2][2] = 'O';
        assert_eq!(check_winner(&board), false);
    }

    #[test]
    fn test_make_move() {
        let mut board = [[' '; 3]; 3];

        // Test valid move
        assert_eq!(make_move(&mut board, 0, 0, 'X'), Ok(()));
        assert_eq!(board[0][0], 'X');

        // Test position already taken
        assert_eq!(make_move(&mut board, 0, 0, 'O'), Err("Position already taken"));
        assert_eq!(board[0][0], 'X'); // Should remain unchanged

        // Test out of bounds
        assert_eq!(make_move(&mut board, 3, 0, 'O'), Err("Invalid position: out of bounds"));
        assert_eq!(make_move(&mut board, 0, 3, 'O'), Err("Invalid position: out of bounds"));
    }

    #[test]
    fn test_full_game_sequence() {
        let mut board = [[' '; 3]; 3];

        // X makes a winning pattern on the first row
        assert_eq!(make_move(&mut board, 0, 0, 'X'), Ok(()));
        assert_eq!(check_winner(&board), false);

        assert_eq!(make_move(&mut board, 1, 0, 'O'), Ok(()));
        assert_eq!(check_winner(&board), false);

        assert_eq!(make_move(&mut board, 0, 1, 'X'), Ok(()));
        assert_eq!(check_winner(&board), false);

        assert_eq!(make_move(&mut board, 1, 1, 'O'), Ok(()));
        assert_eq!(check_winner(&board), false);

        assert_eq!(make_move(&mut board, 0, 2, 'X'), Ok(()));
        assert_eq!(check_winner(&board), true); // X should win
    }

    #[test]
    fn test_draw_game() {
        let mut board = [[' '; 3]; 3];

        // Fill the board with a pattern that results in a draw
        // X O X
        // X O X
        // O X O
        make_move(&mut board, 0, 0, 'X').unwrap();
        make_move(&mut board, 0, 1, 'O').unwrap();
        make_move(&mut board, 0, 2, 'X').unwrap();

        make_move(&mut board, 1, 0, 'X').unwrap();
        make_move(&mut board, 1, 1, 'O').unwrap();
        make_move(&mut board, 1, 2, 'X').unwrap();

        make_move(&mut board, 2, 0, 'O').unwrap();
        make_move(&mut board, 2, 1, 'X').unwrap();
        make_move(&mut board, 2, 2, 'O').unwrap();

        assert_eq!(check_winner(&board), false); // Should be a draw
    }
}
