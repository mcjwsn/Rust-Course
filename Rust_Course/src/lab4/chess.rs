#[derive(Debug)]
enum ChessFigure{
    Pawn, Knight, Bishop, Rook, Queen, King
}
#[derive(Debug)]
enum Color {
    White, Black
}
#[derive(Debug)]
struct Position {
    x: u8,
    y: u8,
}

#[derive(Debug)]
struct ChessPiece {
    figure: ChessFigure,
    position: Position,
    color: Color,
}

impl ChessPiece {
    fn new(figure: ChessFigure, position: Position, color: Color) -> Self {
        ChessPiece {
            figure,
            position,
            color,
        }
    }

    fn move_piece(&mut self, new_position: Position) -> bool {
        if self.is_valid_move(&new_position) {
            self.position = new_position;
            true
        } else {
            false
        }
    }

    fn is_valid_move(&self, new_position: &Position) -> bool {
        let dx = new_position.x.abs_diff(self.position.x);
        let dy = new_position.y.abs_diff(self.position.y);

        match self.figure {
            ChessFigure::Pawn => {
                // Pionki poruszają się tylko do przodu (w górę dla białych, w dół dla czarnych)
                let direction = match self.color {
                    Color::White => 1,
                    Color::Black => -1,
                };

                // Sprawdź czy ruch jest w dobrym kierunku
                let y_diff = new_position.y as i8 - self.position.y as i8;
                if (y_diff != direction && y_diff != direction * 2 && self.position.y != 1 && self.position.y != 6)
                    || (y_diff == direction * 2 && (self.position.y != 1 && self.position.y != 6)) {
                    return false;
                }

                // Pionki poruszają się tylko prosto (x się nie zmienia), chyba że biją
                if dx == 0 {
                    // Normalny ruch do przodu
                    true
                } else if dx == 1 && dy == 1 {
                    // Bicie po skosie (tutaj powinno się jeszcze sprawdzić czy jest tam bierka przeciwnika)
                    true
                } else {
                    false
                }
            },
            ChessFigure::Knight => {
                // Skoczek porusza się w kształcie litery L
                (dx == 1 && dy == 2) || (dx == 2 && dy == 1)
            },
            ChessFigure::Bishop => {
                // Goniec porusza się po skosie
                dx == dy
            },
            ChessFigure::Rook => {
                // Wieża porusza się prosto
                dx == 0 || dy == 0
            },
            ChessFigure::Queen => {
                // Królowa łączy ruchy wieży i gońca
                dx == 0 || dy == 0 || dx == dy
            },
            ChessFigure::King => {
                // Król porusza się o jedno pole w dowolnym kierunku
                dx <= 1 && dy <= 1
            },
        }}


}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_pawn_move_white_forward() {
        let mut pawn = ChessPiece::new(
            ChessFigure::Pawn,
            Position { x: 4, y: 1 },
            Color::White,
        );
        assert!(pawn.move_piece(Position { x: 4, y: 2 }));
        assert_eq!(pawn.position.y, 2);
    }

    #[test]
    fn test_pawn_move_black_forward() {
        let mut pawn = ChessPiece::new(
            ChessFigure::Pawn,
            Position { x: 4, y: 6 },
            Color::Black,
        );
        assert!(pawn.move_piece(Position { x: 4, y: 5 }));
        assert_eq!(pawn.position.y, 5);
    }

    #[test]
    fn test_pawn_invalid_move_sideways() {
        let pawn = ChessPiece::new(
            ChessFigure::Pawn,
            Position { x: 4, y: 1 },
            Color::White,
        );
        assert!(!pawn.is_valid_move(&Position { x: 5, y: 1 }));
    }

    #[test]
    fn test_knight_valid_moves() {
        let knight = ChessPiece::new(
            ChessFigure::Knight,
            Position { x: 4, y: 4 },
            Color::White,
        );
        assert!(knight.is_valid_move(&Position { x: 6, y: 5 }));
        assert!(knight.is_valid_move(&Position { x: 5, y: 6 }));
        assert!(!knight.is_valid_move(&Position { x: 4, y: 6 }));
    }

    #[test]
    fn test_bishop_valid_and_invalid_moves() {
        let bishop = ChessPiece::new(
            ChessFigure::Bishop,
            Position { x: 2, y: 0 },
            Color::White,
        );
        assert!(bishop.is_valid_move(&Position { x: 5, y: 3 }));
        assert!(!bishop.is_valid_move(&Position { x: 2, y: 3 }));
    }

    #[test]
    fn test_rook_valid_and_invalid_moves() {
        let rook = ChessPiece::new(
            ChessFigure::Rook,
            Position { x: 0, y: 0 },
            Color::White,
        );
        assert!(rook.is_valid_move(&Position { x: 0, y: 7 }));
        assert!(!rook.is_valid_move(&Position { x: 7, y: 7 }));
    }

    #[test]
    fn test_queen_valid_and_invalid_moves() {
        let queen = ChessPiece::new(
            ChessFigure::Queen,
            Position { x: 3, y: 3 },
            Color::White,
        );
        assert!(queen.is_valid_move(&Position { x: 3, y: 6 })); // vertical
        assert!(queen.is_valid_move(&Position { x: 6, y: 6 })); // diagonal
        assert!(!queen.is_valid_move(&Position { x: 5, y: 6 })); // invalid
    }

    #[test]
    fn test_king_valid_and_invalid_moves() {
        let king = ChessPiece::new(
            ChessFigure::King,
            Position { x: 4, y: 4 },
            Color::White,
        );
        assert!(king.is_valid_move(&Position { x: 5, y: 5 }));
        assert!(king.is_valid_move(&Position { x: 4, y: 5 }));
        assert!(!king.is_valid_move(&Position { x: 6, y: 6 }));
    }
}


