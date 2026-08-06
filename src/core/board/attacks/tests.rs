#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn verify_knight_moves() {
        let knight_bitboard = knight::knight_attacks_from_square(0);
        assert_eq!(knight_bitboard, 0x20400);
        let knight_bitboard = knight::knight_attacks_from_square(27);
        assert_eq!(knight_bitboard, 0x142200221400);
    }

    #[test]
    fn verify_king_moves() {
        let king_bitboard = king::king_attacks_from_square(0);
        assert_eq!(king_bitboard, 0x302);
        let king_bitboard = king::king_attacks_from_square(27);
        assert_eq!(king_bitboard, 0x1C141C0000);
    }

    #[test]
    fn movegen_init() {
        let king_moves = king::init_king_attacks();
        assert_eq!(king_moves.len(), 64);

        let knight_moves = knight::init_knight_attacks();
        assert_eq!(knight_moves.len(), 64);
    }
}
