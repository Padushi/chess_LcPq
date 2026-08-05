use crate::core::board::attacks::{FILE_A, FILE_H};

const RANK_4: u64 = 0xFF000000;

pub fn white_pawn_attacks_from_square(sq: usize) -> u64 {
    let bitboard: u64 = 1 << sq;
    let mut attacks: u64 = 0;

    attacks |= (bitboard << 9) & !FILE_A;
    attacks |= (bitboard << 7) & !FILE_H;
    attacks
}

pub fn black_pawn_attacks_from_square(sq: usize) -> u64 {
    let bitboard: u64 = 1 << sq;
    let mut attacks: u64 = 0;
    
    attacks |= (bitboard >> 7) & !FILE_A;
    attacks |= (bitboard >> 9) & !FILE_H;
    attacks
}

pub fn white_pawn_single_pushes(white_pawns: u64, occupied: u64) -> u64 {
    (white_pawns << 8) & !occupied
}

pub fn white_pawn_double_pushes(white_pawns: u64, occupied: u64) -> u64 {
    let single_pushes = white_pawn_single_pushes(white_pawns, occupied);
    (single_pushes << 8) & !occupied & RANK_4
}