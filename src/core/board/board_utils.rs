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

pub fn index_from_coordinates(coords: &str) -> usize {
    let file = coords.chars().nth(0).unwrap() as usize - 97;
    let rank = coords.chars().nth(1).unwrap().to_digit(10).unwrap() as usize;

    return (rank - 1) * 8 + file;
}

pub fn coordinates_from_index(index: usize) -> String {
    let file = ((97 + index % 8) as u8) as char;
    let rank = index / 8 + 1;

    return format!("{file}{rank}");
}
