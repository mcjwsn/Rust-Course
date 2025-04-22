pub fn check_sudoku_board(board: [[i8; 9]; 9]) -> bool {
    if !valid_data(&board) || !valid_row(&board) || !valid_column(&board) || !valid_squares(&board) {return false}
    true
}
fn valid_data(board:&[[i8; 9]; 9]) -> bool {
    for i in 0..9{
        for j in 0..9{
            if board[i][j] > 9 || board[i][j] < 0{return false;}
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

fn valid_squares(board:&[[i8; 9]; 9]) -> bool {
    let mids = [(1,1),(4,4),(7,7),(1,4),(1,7),(4,1),(4,7),(7,1),(7,4)];
    let neighbours: [(i32, i32); 9] = [(1,1),(0,1),(-1,1),(-1,0),(-1,-1),(0,-1),(1,0),(1,-1),(0,0)];
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


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_data() {
        // Valid board with only values 0-9
        let valid_board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(valid_data(&valid_board));

        // Invalid board with value > 9
        let invalid_board_high = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 10, 0, 0, 0],  // 10 is invalid
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!valid_data(&invalid_board_high));

        // Invalid board with negative value
        let invalid_board_negative = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, -1, 0, 0, 2, 0, 0, 0, 6],  // -1 is invalid
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!valid_data(&invalid_board_negative));
    }

    #[test]
    fn test_valid_row() {
        // Valid board with no duplicates in rows
        let valid_board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(valid_row(&valid_board));

        // Invalid board with duplicate in row
        let invalid_board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 6, 0, 1, 9, 5, 0, 0, 0],  // Two 6s in the same row
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!valid_row(&invalid_board));
    }

    #[test]
    fn test_valid_column() {
        // Valid board with no duplicates in columns
        let valid_board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(valid_column(&valid_board));

        // Invalid board with duplicate in column
        let invalid_board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [8, 0, 0, 4, 1, 9, 0, 0, 5],  // 8 is duplicate in column 0
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!valid_column(&invalid_board));
    }

    #[test]
    fn test_valid_squares() {
        // Valid board with no duplicates in 3x3 squares
        let valid_board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(valid_squares(&valid_board));

        // Invalid board with duplicate in a 3x3 square
        let invalid_board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 5, 0, 1, 9, 5, 0, 0, 0],  // 5 is duplicate in top-left 3x3 square
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!valid_squares(&invalid_board));
    }

    #[test]
    fn test_check_sudoku_board() {
        // Valid incomplete board
        let valid_board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(check_sudoku_board(valid_board));

        // Complete valid solution
        let valid_solution = [
            [5, 3, 4, 6, 7, 8, 9, 1, 2],
            [6, 7, 2, 1, 9, 5, 3, 4, 8],
            [1, 9, 8, 3, 4, 2, 5, 6, 7],
            [8, 5, 9, 7, 6, 1, 4, 2, 3],
            [4, 2, 6, 8, 5, 3, 7, 9, 1],
            [7, 1, 3, 9, 2, 4, 8, 5, 6],
            [9, 6, 1, 5, 3, 7, 2, 8, 4],
            [2, 8, 7, 4, 1, 9, 6, 3, 5],
            [3, 4, 5, 2, 8, 6, 1, 7, 9]
        ];
        assert!(check_sudoku_board(valid_solution));

        // Invalid board with duplicate in row
        let invalid_row_board = [
            [5, 5, 0, 0, 7, 0, 0, 0, 0],  // Two 5s in the same row
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!check_sudoku_board(invalid_row_board));

        // Invalid board with duplicate in column
        let invalid_column_board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [5, 9, 8, 0, 0, 0, 0, 6, 0],  // 5 is duplicate in column 0
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!check_sudoku_board(invalid_column_board));

        // Invalid board with duplicate in 3x3 square
        let invalid_square_board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [9, 9, 8, 0, 0, 0, 0, 6, 0],  // Two 9s in top-left 3x3 square
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 3, 0, 0, 1],
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!check_sudoku_board(invalid_square_board));

        // Invalid board with value > 9
        let invalid_value_board = [
            [5, 3, 0, 0, 7, 0, 0, 0, 0],
            [6, 0, 0, 1, 9, 5, 0, 0, 0],
            [0, 9, 8, 0, 0, 0, 0, 6, 0],
            [8, 0, 0, 0, 6, 0, 0, 0, 3],
            [4, 0, 0, 8, 0, 10, 0, 0, 1],  // 10 is invalid
            [7, 0, 0, 0, 2, 0, 0, 0, 6],
            [0, 6, 0, 0, 0, 0, 2, 8, 0],
            [0, 0, 0, 4, 1, 9, 0, 0, 5],
            [0, 0, 0, 0, 8, 0, 0, 7, 9]
        ];
        assert!(!check_sudoku_board(invalid_value_board));
    }
}