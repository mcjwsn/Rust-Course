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
