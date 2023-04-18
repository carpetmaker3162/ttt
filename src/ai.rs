use ttt::{
    Board,
    Player,
    has_2filled,
    has_2filled_by,
};

fn get_empty_squares(board: &Board) -> Vec<(usize, usize)> {
    let mut empty = Vec::new();
    for (i, row) in board.squares.iter().enumerate() {
        for (j, &cell) in row.iter().enumerate() {
            if cell.is_none() {
                empty.push((i, j));
            }
        }
    }
    empty
}

fn check_for_win(board: &Board) -> Option<(usize, usize)> {
    for i in 0..3 {
        let row = board.squares[i];
        if let Some(x) = has_2filled(row) {
            return Some((i, x));
        }

        let col = [board.squares[0][i], board.squares[1][i], board.squares[2][i]];
        if let Some(x) = has_2filled(col) {
            return Some((x, i));
        }
    }

    let diagonal1 = [board.squares[0][0], board.squares[1][1], board.squares[2][2]];
    let diagonal2 = [board.squares[0][2], board.squares[1][1], board.squares[2][0]];

    if let Some(x) = has_2filled(diagonal1) {
        return Some((x, x));
    }

    if let Some(x) = has_2filled(diagonal2) {
        return Some((x, 2 - x));
    }

    None
}

fn position_has_fork(grid: [[Option<Player>; 3]; 3], player: Player) -> bool {
    let mut count = 0;

    for i in 0..3 {
        let row = grid[i];
        if has_2filled_by(row, player).is_some() {
            count += 1;
        }

        let col = [grid[0][i], grid[1][i], grid[2][i]];
        if has_2filled_by(col, player).is_some() {
            count += 1;
        }
    }

    let diagonal1 = [grid[0][0], grid[1][1], grid[2][2]];
    let diagonal2 = [grid[0][2], grid[1][1], grid[2][0]];
    if has_2filled(diagonal1).is_some() { count += 1 };
    if has_2filled(diagonal2).is_some() { count += 1 };

    count > 1
}

// check board for potential forks
fn check_for_fork(board: &Board, turn: Player) -> Option<(usize, usize)> {
    let empty_squares = get_empty_squares(&board);
    for (r, c) in empty_squares {
        let new_board = board.get_board_with_placement(turn, r, c);
        if position_has_fork(new_board, turn) {
            return Some((r, c));
        }
    }
    None
}

// return the best move for a player in a given board
pub fn best_move(board: &Board, turn: Player) -> (usize, usize) {
    let other_player = match turn {
        Player::X => Player::O,
        Player::O => Player::X
    };
    let corners = vec![(0, 0), (0, 2), (2, 0), (2, 2)];
    let sides = vec![(0, 1), (1, 0), (2, 1), (1, 2)];

    // check for win or block
    if let Some(x) = check_for_win(board) {
        return x;
    }
    
    // check for own forks
    if let Some(x) = check_for_fork(board, turn) {
        return x;
    }

    // check for enemy fork
    if let Some(x) = check_for_fork(board, other_player) {
        return x;
    }

    if turn == Player::X && get_empty_squares(&board).len() == 9 {
        return (0, 0);
    }

    if board.squares[1][1].is_none() {
        return (1, 1);
    }
    
    // play opposite corner
    for (r, c) in corners.iter() {
        let idx_r = r.clone();
        let idx_c = c.clone();
        if board.squares[idx_r][idx_c] == Some(other_player) {
            if board.squares[2 - idx_r][2 - idx_c].is_none() {
                return (2 - idx_r, 2 - idx_c);
            }
        }
    }

    // play empty corner
    for (r, c) in corners.iter() {
        let idx_r = r.clone();
        let idx_c = c.clone();
        if board.squares[idx_r][idx_c].is_none() {
            return (idx_r, idx_c);
        }
    }

    // play empty side square
    for (r, c) in sides.iter() {
        let idx_r = r.clone();
        let idx_c = c.clone();
        if board.squares[idx_r][idx_c].is_none() {
            return (idx_r, idx_c);
        }       
    }

    return (0, 0);
}
