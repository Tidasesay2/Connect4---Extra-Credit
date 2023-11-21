#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_make_move() {
        let mut game = Connect4::new();

        // Test making valid moves
        assert_eq!(game.make_move(3), Ok(()));
        assert_eq!(game.make_move(4), Ok(()));
        assert_eq!(game.make_move(2), Ok(()));
        assert_eq!(game.make_move(1), Ok(()));

        // Test making moves in an invalid column
        assert_eq!(game.make_move(0), Err("Invalid column number"));
        assert_eq!(game.make_move(8), Err("Invalid column number"));

        // Test making moves in a full column
        for _ in 0..6 {
            game.make_move(3).unwrap();
        }
        assert_eq!(game.make_move(3), Err("Column is full"));
    }

    #[test]
    fn test_switch_player() {
        let mut game = Connect4::new();

        // Test switching player from X to O
        game.switch_player();
        assert_eq!(game.current_player, 'O');

        // Test switching player from O to X
        game.switch_player();
        assert_eq!(game.current_player, 'X');

        // Test multiple player switches
        game.switch_player();
        game.switch_player();
        game.switch_player();
        assert_eq!(game.current_player, 'O');
    }

    #[test]
    fn test_check_winner() {
        // Test horizontal win
        let mut game = Connect4::new();
        game.make_move(1).unwrap();
        game.make_move(2).unwrap();
        game.make_move(3).unwrap();
        game.make_move(4).unwrap();
        assert_eq!(game.check_winner(), None);

        game.make_move(5).unwrap();
        assert_eq!(game.check_winner(), Some('X'));

        // Test vertical win
        let mut game = Connect4::new();
        game.make_move(1).unwrap();
        game.make_move(1).unwrap();
        game.make_move(1).unwrap();
        game.make_move(1).unwrap();
        assert_eq!(game.check_winner(), Some('X'));

        // Test diagonal win (top-left to bottom-right)
        let mut game = Connect4::new();
        game.make_move(1).unwrap();
        game.make_move(2).unwrap();
        game.make_move(2).unwrap();
        game.make_move(3).unwrap();
        game.make_move(3).unwrap();
        game.make_move(3).unwrap();
        game.make_move(4).unwrap();
        game.make_move(4).unwrap();
        game.make_move(4).unwrap();
        game.make_move(4).unwrap();
        assert_eq!(game.check_winner(), Some('X'));

        // Test diagonal win (bottom-left to top-right)
        let mut game = Connect4::new();
        game.make_move(7).unwrap();
        game.make_move(6).unwrap();
        game.make_move(6).unwrap();
        game.make_move(5).unwrap();
        game.make_move(5).unwrap();
        game.make_move(5).unwrap();
        game.make_move(4).unwrap();
        game.make_move(4).unwrap();
        game.make_move(4).unwrap();
        game.make_move(4).unwrap();
        assert_eq!(game.check_winner(), Some('X'));

        // Test no winner
        let mut game = Connect4::new();
        game.make_move(1).unwrap();
        game.make_move(2).unwrap();
        game.make_move(1).unwrap();
        game.make_move(2).unwrap();
        game.make_move(1).unwrap();
        game.make_move(2).unwrap();
        game.make_move(3).unwrap();
        assert_eq!(game.check_winner(), None);
    }

    // Add more test cases as needed
}
