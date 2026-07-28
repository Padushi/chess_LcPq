pub struct Board {
    // Normal Info
    state: Vec<char>,
    active_color: bool, // true for white, false for black
    castling: String,
    en_passant: String,
    halfmove: usize,
    fullmove: usize,

    // Bitboards
    white_king: u64,
    white_queen: u64,
    white_rook: u64,
    white_bishop: u64,
    white_knight: u64,
    white_pawn: u64,

    black_king: u64,
    black_queen: u64,
    black_rook: u64,
    black_bishop: u64,
    black_knight: u64,
    black_pawn: u64,
}

pub fn get_state_from_fen(fen: &str) -> Vec<char> {
    let mut state: Vec<char> = vec![' '; 64];

    let files: Vec<&str> = fen.split('/').rev().collect();
    let mut n: usize = 0;

    for file in files {
        for i in file.chars() {
            if i.is_digit(10) {
                println!("{} is a number", i);
                n += i.to_digit(10).unwrap() as usize;
            } else if i.is_alphabetic() {
                println!("{} is a piece", i);
                state[n] = i;
                n += 1
            } else {
                n += 1
            }
        }
    }

    state
}

pub fn build_board_from_fen(fen: &str) -> Board {
    let fen_areas: Vec<&str> = fen.split(' ').collect();

    Board {
        state: get_state_from_fen(fen_areas[0]),
        active_color: match fen_areas[1] {
            "w" => true,
            "b" => false,
            &_ => todo!("Active color is neither white(w) nor black(b)"),
        },
        castling: String::from("KQkq"),
        en_passant: String::from("e3"),
        halfmove: 0,
        fullmove: 0,
        white_king: 0,
        white_queen: 0,
        white_rook: 0,
        white_bishop: 0,
        white_knight: 0,
        white_pawn: 0,
        black_king: 0,
        black_queen: 0,
        black_rook: 0,
        black_bishop: 0,
        black_knight: 0,
        black_pawn: 0,
    }
}
