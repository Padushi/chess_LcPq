use crate::core::board::attacks::{FILE_A, FILE_H};

pub fn knight_attacks_from_square(sq: usize) -> u64 {
    const FILE_B: u64 = 0x202020202020202;
    const FILE_G: u64 = 0x4040404040404040;

    let bitboard: u64 = 1 << sq;
    let mut attacks: u64 = 0;

    attacks |= (bitboard << 17) & !FILE_A;
    attacks |= (bitboard << 15) & !FILE_H;
    attacks |= (bitboard << 10) & !(FILE_A | FILE_B);
    attacks |= (bitboard << 6) & !(FILE_G | FILE_H);
    attacks |= (bitboard >> 17) & !FILE_H;
    attacks |= (bitboard >> 15) & !FILE_A;
    attacks |= (bitboard >> 10) & !(FILE_G | FILE_H);
    attacks |= (bitboard >> 6) & !(FILE_A | FILE_B);

    attacks
}

pub fn init_knight_attacks() -> [u64; 64] {
    let bitboards: [u64; 64] = std::array::from_fn(knight_attacks_from_square);
    bitboards
}

