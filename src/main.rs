use ttt::Board;
use ttt::Player;
use std::io::Write;

fn main() {
    let mut board = Board::new();
    println!("welcome!");
    println!("provide movves in the form of \"row# col#\"");
    loop {
        board.print();
        print!(">>> ");
        std::io::stdout().flush().unwrap();
        let (r, c) = Board::get_move();
        if let Err(x) = board.make_move(r - 1, c - 1) {
            eprintln!("ERROR: {}", x);
        }
        if board.has_won(Player::X) { 
            println!("X WINS GG EZ");
            break;
        }
        if board.has_won(Player::O) {
            println!("O WINS GG EZ");
            break;
        }
        if board.squares.iter().all(|row| 
                row.iter().all(|cell|
                    cell.is_some())) {
            println!("you all suck");
            break;
        }
    }
}