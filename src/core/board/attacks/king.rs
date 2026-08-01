use crate::core::board::attacks::{FILE_A, FILE_H};

pub fn king_attacks_from_square(sq: usize) -> u64 {
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

pub fn init_king_attacks() -> [u64; 64] {
    let bitboards: [u64; 64] = std::array::from_fn(king_attacks_from_square);
    bitboards
}

