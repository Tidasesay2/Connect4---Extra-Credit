use std::io;


fn main() {
    let mut game = Connect4::new();
      loop {
        game.displaycrate();
        println!("select where you wish to place your token (1-7)");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed input try again!");

        let column: usize = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Enter a number! anything ovr 7 is invalid!");
                continue;
            }
        };

 match game.make_move(column) {
    Ok(_) => {
            
    if let Some(winner) = game.check_winner() {
         game.displaycrate();
         println!("{} is the winner woohoo!", game.get_player_name(winner));
        break;
                }

                game.swap_player();
            }
            Err(err) => println!("{}", err),
        }
    }


}

const ROWS: usize = 6;
const COLS: usize = 7;

struct Connect4 {
    board: [[usize; COLS]; ROWS],
    current_player: usize,
}

impl Connect4 {
    fn new() -> Connect4 {
        Connect4 {
            board: [[0; COLS]; ROWS],
            current_player: 1,
        }
    }

    fn displaycrate(&self) { //the connect 4 crate 
        for row in &self.board {
            for &cell in row {
                print!("{:<7}", self.get_player_name(cell)); // Adjusted spacing
            }
            println!();
        }

        for col_num in 1..=COLS {
            print!("{:<7}", col_num); 
        }

        println!("\n{}'s turn.", self.get_current_player_name());
    }

 fn make_move(&mut self, column: usize) -> Result<(), &str> {
     if column < 1 || column > COLS {
         return Err("Invalid column number");
        }

     for row in (0..ROWS).rev() {
        if self.board[row][column - 1] == 0 {
              self.board[row][column - 1] = self.current_player;
                return Ok(());
            }
        }

        Err("Column is full")
    }

    fn swap_player(&mut self) {
        self.current_player = 3 - self.current_player; // alternating  players
    }

    fn check_winner(&self) -> Option<usize> {
        //checking the row
        for row in 0..ROWS {
            for col in 0..COLS - 3 {
                if self.board[row][col] != 0
                    && self.board[row][col] == self.board[row][col + 1]
                    && self.board[row][col] == self.board[row][col + 2]
                    && self.board[row][col] == self.board[row][col + 3]
                {
                    return Some(self.board[row][col]);
                }
            }
        }

        // now Checking the columns 
 for col in 0..COLS {
    for row in 0..ROWS - 3 {
    if self.board[row][col] != 0
    && self.board[row][col] == self.board[row + 1][col]
    && self.board[row][col] == self.board[row + 2][col]
    && self.board[row][col] == self.board[row + 3][col]
      {
        return Some(self.board[row][col]);
                }
            }
        }

 // diagonal checks 
    for row in 0..ROWS - 3 {
    for col in 0..COLS - 3 {

    if self.board[row][col] != 0
        && self.board[row][col] == self.board[row + 1][col + 1]
        && self.board[row][col] == self.board[row + 2][col + 2]
        && self.board[row][col] == self.board[row + 3][col + 3]
                {
                    return Some(self.board[row][col]);
                }
            }
        }

       
        for row in 3..ROWS {
            for col in 0..COLS - 3 {

                if self.board[row][col] != 0
                    && self.board[row][col] == self.board[row - 1][col + 1]
                    && self.board[row][col] == self.board[row - 2][col + 2]
                    && self.board[row][col] == self.board[row - 3][col + 3]
                {
                    return Some(self.board[row][col]);
                }
            }
        }


        None
    }

    fn get_player_name(&self, player: usize) -> &str {

        match player { // these will show in the crate 
            1 => "Red",
            2 => "Yellow",
            _ => "Empty", // so player knows what rows are available to use 
        }
    }


    fn get_current_player_name(&self) -> &str {
        self.get_player_name(self.current_player)
    }
}
