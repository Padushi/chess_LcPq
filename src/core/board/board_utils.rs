use super::board_core::Board;

pub fn print_board(board: &Board, white_at_bottom: bool) {
    println!();

    match white_at_bottom {
        true => {
            println!("    a   b   c   d   e   f   g   h");
            println!("  +---+---+---+---+---+---+---+---+");
            for i in 0..8 {
                println!(
                    "{} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
                    8 - i,
                    board.state[63 - 8 * i - 7],
                    board.state[63 - 8 * i - 6],
                    board.state[63 - 8 * i - 5],
                    board.state[63 - 8 * i - 4],
                    board.state[63 - 8 * i - 3],
                    board.state[63 - 8 * i - 2],
                    board.state[63 - 8 * i - 1],
                    board.state[63 - 8 * i],
                    8 - i,
                );
                println!("  +---+---+---+---+---+---+---+---+");
            }
            println!("    a   b   c   d   e   f   g   h");
        }
        false => {
            println!("    h   g   f   e   d   c   c   b   a");
            println!("  +---+---+---+---+---+---+---+---+");
            for i in 0..8 {
                println!(
                    "{} | {} | {} | {} | {} | {} | {} | {} | {} | {}",
                    i + 1,
                    board.state[8 * i + 7],
                    board.state[8 * i + 6],
                    board.state[8 * i + 5],
                    board.state[8 * i + 4],
                    board.state[8 * i + 3],
                    board.state[8 * i + 2],
                    board.state[8 * i + 1],
                    board.state[8 * i],
                    i + 1,
                );
                println!("  +---+---+---+---+---+---+---+---+");
            }
            println!("    h   g   f   e   d   c   c   b   a");
        }
    }
}
