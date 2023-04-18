use std::io::Write;
use ttt::{
    Board,
    Player
};

mod ai;

fn main() {
    let mut board = Board::new();
    println!("welcome!");
    println!("provide movves in the form of \"row# col#\"");
    loop {
        let (ai_r, ai_c) = ai::best_move(&board, board.turn);
        if let Err(x) = board.make_move(ai_r, ai_c) {
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
        std::io::stdout().flush().unwrap();
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
