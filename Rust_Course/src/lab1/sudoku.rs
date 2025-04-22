pub fn check_sudoku_board(board: [[i8; 9]; 9]) -> bool {
    if !valid_data(&board) || !valid_row(&board) || !valid_column(&board) || !valid_sqaures(&board) {return false}
    true
}
fn valid_data(board:&[[i8; 9]; 9]) -> bool {
    for i in 0..9{
        for j in 0..9{
            if board[i][j] > 9 {return false;}
        }
    }
    true
}

fn valid_row(board:&[[i8; 9]; 9]) -> bool{
    let mut tab;
    for i in 0..9{
        tab = [1;9];
        for j in 0..9{
            if board[i][j] != 0 {
                if tab[(board[i][j]-1) as usize] == 2{return false;}
                tab[(board[i][j]-1) as usize] += 1;
            };
        }
    }
    true
}

fn valid_column(board:&[[i8; 9]; 9]) -> bool{
    let mut tab;
    for i in 0..9{
        tab = [1;9];
        for j in 0..9{
            if board[j][i] != 0 {
                if tab[(board[j][i]-1) as usize] == 2{return false;}
                tab[(board[j][i]-1) as usize] += 1;
            };
        }
    }
    true
}

fn valid_sqaures(board:&[[i8; 9]; 9]) -> bool {
    let mids = [(1,1),(4,4),(7,7),(1,4),(1,7),(4,1),(4,7),(7,1),(7,4)];
    let neighbours: [(i32, i32); 8] = [(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),(1,0),(1,-1)];
    let mut tab;
    for start in &mids{
        let (x,y) = start;
        tab = [1;9];
        for n in &neighbours{
            let (a,b) = n;
            if board[(x+a) as usize][(y+b) as usize] != 0 {
                if tab[(board[(x+a) as usize][(y+b) as usize]-1) as usize] == 2{return false;}
                tab[(board[(x+a) as usize][(y+b) as usize]-1) as usize] += 1;
            };
        }
    }
    true
}