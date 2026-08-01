use crate::core::board::attacks::{FILE_A, FILE_H};

pub fn king_attacks_from_square(sq: usize) -> u64 {
    const FILE_B: u64 = 0x202020202020202;
    const FILE_G: u64 = 0x4040404040404040;

    let bitboard: u64 = 1 << sq;
    let mut attacks: u64 = 0;

    attacks |= bitboard << 8;
    attacks |= (bitboard << 1) & !FILE_A;
    attacks |= (bitboard << 9) & !FILE_H;
    attacks |= (bitboard << 7) & !FILE_H;
    attacks |= bitboard >> 8;
    attacks |= (bitboard >> 1) & !FILE_H;
    attacks |= (bitboard >> 9) & !FILE_H;
    attacks |= (bitboard >> 7) & !FILE_A;

    attacks
}

