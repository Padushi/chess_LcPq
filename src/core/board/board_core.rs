#[allow(dead_code)]
pub struct Board {
    // Normal Info
    pub state: Vec<char>,
    pub active_color: bool, // true for white, false for black
    pub castling: String,
    pub en_passant: String,
    pub halfmove: usize,
    pub fullmove: usize,

    // Bitboards
    pub white_king: u64,
    pub white_queen: u64,
    pub white_rook: u64,
    pub white_bishop: u64,
    pub white_knight: u64,
    pub white_pawn: u64,

    pub black_king: u64,
    pub black_queen: u64,
    pub black_rook: u64,
    pub black_bishop: u64,
    pub black_knight: u64,
    pub black_pawn: u64,
}

pub fn get_state_from_fen(fen: &str) -> Vec<char> {
    let mut state: Vec<char> = vec![' '; 64];

    let files: Vec<&str> = fen.split('/').rev().collect();
    let mut n: usize = 0;

    for file in files {
        for i in file.chars() {
            if i.is_digit(10) {
                n += i.to_digit(10).unwrap() as usize;
            } else if i.is_alphabetic() {
                state[n] = i;
                n += 1
            } else {
                n += 1
            }
        }
    }

    state
}

impl Board {
    pub fn new_from_fen(fen: &str) -> Board {
        use crate::core::board::bitboards;

        let fen_areas: Vec<&str> = fen.split(' ').collect();

        let state = get_state_from_fen(fen_areas[0]);

        Board {
            state: state.clone(),
            active_color: match fen_areas[1] {
                "w" => true,
                "b" => false,
                &_ => panic!("Active color is neither white(w) nor black(b)"),
            },
            castling: String::from("KQkq"),
            en_passant: String::from("e3"),
            halfmove: 0,
            fullmove: 0,

            white_king: bitboards::bitboard_from_char('K', &state),
            white_queen: bitboards::bitboard_from_char('Q', &state),
            white_rook: bitboards::bitboard_from_char('R', &state),
            white_bishop: bitboards::bitboard_from_char('B', &state),
            white_knight: bitboards::bitboard_from_char('N', &state),
            white_pawn: bitboards::bitboard_from_char('P', &state),

            black_king: bitboards::bitboard_from_char('k', &state),
            black_queen: bitboards::bitboard_from_char('q', &state),
            black_rook: bitboards::bitboard_from_char('r', &state),
            black_bishop: bitboards::bitboard_from_char('b', &state),
            black_knight: bitboards::bitboard_from_char('n', &state),
            black_pawn: bitboards::bitboard_from_char('p', &state),
        }
    }

    pub fn white_occupancy(&self) -> u64 {
        return self.white_king
            | self.white_queen
            | self.white_rook
            | self.white_bishop
            | self.white_knight
            | self.white_pawn;
    }

    pub fn black_occupancy(&self) -> u64 {
        return self.black_king
            | self.black_queen
            | self.black_rook
            | self.black_bishop
            | self.black_knight
            | self.black_pawn;
    }

    pub fn all_occupancy(&self) -> u64 {
        return self.white_occupancy() | self.black_occupancy();
    }
}
