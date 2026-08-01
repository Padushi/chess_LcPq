pub fn bitboard_from_char(piece: char, board_state: &Vec<char>) -> u64 {
    let mut bitboard: u64 = 0b0;

    for i in 0..board_state.len() {
        if board_state[i] == piece {
            bitboard ^= 1 << i;
        }
    }
    bitboard
}

pub fn print_bitboard(bitboard: u64) {
    let rank = bitboard.reverse_bits().to_le_bytes();
    for j in rank {
        println!("{:08b}", j);
    }
}
