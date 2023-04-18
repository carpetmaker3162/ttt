use ttt::Board;
use ttt::Player;
use std::io::Write;

mod ai;

fn main() {
    let mut board = Board::new();
    board.make_move(0, 0);
    board.make_move(1, 0);
    board.make_move(0, 2);
    board.make_move(0, 1);
    board.print();
    println!("{:?}", board.arr_has_2filled(board.squares[0], Player::X));
}

fn _main() {
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
