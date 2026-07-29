use super::board_core::Board;

pub fn print_board(board: &Board, with_piece_symbols: bool, white_at_bottom: bool) {
    println!();
    println!("    a   b   c   d   e   f   g   h");
    println!("  +---+---+---+---+---+---+---+---+");
    for i in 0..8 {
        println!(
            "{} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
            8 - i,
            board.state[8 * i],
            board.state[8 * i + 1],
            board.state[8 * i + 2],
            board.state[8 * i + 3],
            board.state[8 * i + 4],
            board.state[8 * i + 5],
            board.state[8 * i + 6],
            board.state[8 * i + 7],
            8 - i,
        );
        println!("  +---+---+---+---+---+---+---+---+");
    }
    println!("    a   b   c   d   e   f   g   h");
}
