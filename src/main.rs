use std::io::Write;
use std::io;

const SIZE: usize = 8;
const EMPTY: char = '.';
const BLACK: char = 'B';
const WHITE: char = 'W';
const DIRECTIONS: [(isize, isize); 8] = [(-1, 0), (1, 0), (0, -1), (0, 1),
(-1, -1), (-1, 1), (1, -1), (1, 1)];

#[derive(PartialEq, Eq)]
enum Player{
    Black,
    White,
}

struct Board{
    grid:[[char;SIZE];SIZE],
}
impl Board {
    fn new() -> Self {
        let mut grid =[[EMPTY;SIZE]; SIZE];
        grid[3][3] = WHITE;
        grid[3][4] = BLACK;
        grid[4][3] = BLACK;
        grid[4][4] = WHITE;
        Board { grid }
}


    fn print(&self) {
        println!("  abcdefgh");
        for (i, row) in self.grid.iter().enumerate() {
            print!("{}", (b'a' + i as u8) as char);
             print!(" ");
            for &cell in row.iter() {
                print!("{}",cell);
            }
            println!();
        }
    }

    fn is_valid_move(&self, player:&Player, row:usize, col:usize) -> bool{
        if self.grid[row][col] != EMPTY {
            return false;
        }
        
        let opponent = match player {
            Player::Black => WHITE,
            Player::White => BLACK,
        };

       
        for &(row_delta, col_delta) in DIRECTIONS.iter() {
            //println!("row_delta{},col_delta{}",row_delta, col_delta);
            if self.check_direction(row,col,row_delta,col_delta,player,opponent) {
                return true;
            }
        }
        false
    }

    fn check_direction(&self, row: usize, col: usize, row_delta: isize, col_delta: isize, player: &Player, opponent: char) -> bool {
        let mut r = row as isize + row_delta;
        let mut c = col as isize + col_delta;
        let mut found_opponent = false;

        while r >= 0 && r < SIZE as isize && c >= 0 && c < SIZE as isize {
            let current = self.grid[r as usize][c as usize];

            if current == opponent {
                found_opponent = true;
            }

            else if current == match player {
                Player::Black => BLACK,
                Player::White => WHITE,
            } {
                return found_opponent;
            }

            else {
                return false;
            }


            r += row_delta;
            c += col_delta;
        }


        false
    }

    fn apply_move(&mut self, player:&Player, row:usize, col:usize) {
        let piece = match player {
            Player::Black => BLACK,
            Player::White => WHITE,
        };
        self.grid[row][col] = piece;
        let opponent = match player {
            Player::Black => WHITE,
            Player::White => BLACK,
        };
        for &(row_delta, col_delta) in DIRECTIONS.iter() {
              if self.check_direction(row, col, row_delta, col_delta, player, opponent) {
                self.flip_pieces(row, col, row_delta, col_delta, player, opponent);
               }
        }
    }

    fn flip_pieces(&mut self, row:usize, col:usize, row_delta: isize, col_delta: isize, player: &Player, opponent: char) {
         //print!("rowx{},colx{}",row, col);
        let mut r = row as isize + row_delta;
        let mut c = col as isize + col_delta;

        let player = match player {
            Player::Black => BLACK,
            Player::White => WHITE,
        };

        while self.grid[r as usize][c as usize] == opponent {
            //print!("change r{},c{},row_delta{}, col_delta{}",r,c,row_delta, col_delta);
            self.grid[r as usize][c as usize] = player;
            r += row_delta;
            c += col_delta;
        }
    }

    fn has_valid_move(&self, player: &Player) -> bool {
        for row in 0..SIZE {
            for col in 0..SIZE {
                if self.is_valid_move(&player,row,col) {
                    return true;
                }
            }
        }
        false
    }

    fn count_pieces(&self, piece:char) -> usize {
        self.grid.iter().flatten().filter(|&&p| p==piece).count()
    }

    fn game_over(&self) -> bool {
        !self.has_valid_move(&Player::Black) && !self.has_valid_move(&Player::White)
    }

    fn print_result(&self) {
        let black_count = self.count_pieces(BLACK);
        let white_count = self.count_pieces(WHITE);

        println!("B player has no valid move.");
        println!("W player has no valid move.");
        if black_count > white_count {
            println!("Black wins by {} points!", black_count - white_count);
        } else if white_count > black_count {
            println!("White wins by {} points!", white_count - black_count);
        } else {
            println!("Draw!");
        }
    }
}

fn get_move() -> (usize, usize) {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read input");
    if input.len() != 3{
        return (SIZE, SIZE);
    }
   
    let col = input.chars().nth(1).unwrap();
    let row = input.chars().nth(0).unwrap();
    let col_index = col as usize - 'a' as usize;
    let row_index = row as usize - 'a' as usize;
    (row_index, col_index)
}

fn main() {
    let mut board = Board::new();
    let mut current_player = Player::Black;
    loop {
        board.print();

        let player_name = match current_player {
            Player::Black => "B",
            Player::White => "W"
        };

        if board.game_over() {
            board.print_result();
            break;
        }

        if !board.has_valid_move(&current_player) {
            println!("{} player has no valid move.", player_name);
            current_player = if current_player == Player::Black {Player::White} else {Player::Black};
            continue;
        }

        print!("Enter move for colour {} (RowCol): ", player_name);
        io::stdout().flush().unwrap();

        let (row, col) = get_move();
        if row >= SIZE || col >= SIZE || !board.is_valid_move(&current_player, row, col) {
            println!("Invalid move. Try again.");
            continue;
        }

        board.apply_move(&current_player, row, col);
        current_player = if current_player == Player::Black {Player::White} else {Player::Black};
    }
}
