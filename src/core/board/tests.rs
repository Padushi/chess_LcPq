#[cfg(test)]
mod tests {
    use super::super::*;

    #[test]
    fn bitboard_from_char_handles_all_default_pieces() {
        use super::super::bitboards::bitboard_from_char;
        let default_board_state = board_core::build_board_from_fen(
            "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1",
        )
        .state;

        let white_kings = bitboard_from_char('K', &default_board_state);
        assert_eq!(white_kings, 16);

        let white_queens = bitboard_from_char('Q', &default_board_state);
        assert_eq!(white_queens, 8);

        let white_rooks = bitboard_from_char('R', &default_board_state);
        assert_eq!(white_rooks, 129);

        let white_bishops = bitboard_from_char('B', &default_board_state);
        assert_eq!(white_bishops, 36);

        let white_knights = bitboard_from_char('N', &default_board_state);
        assert_eq!(white_knights, 66);

        let white_pawns = bitboard_from_char('P', &default_board_state);
        assert_eq!(white_pawns, 65280);

        let black_kings = bitboard_from_char('k', &default_board_state);
        assert_eq!(black_kings, 1152921504606846976);

        let black_queens = bitboard_from_char('q', &default_board_state);
        assert_eq!(black_queens, 576460752303423488);

        let black_rooks = bitboard_from_char('r', &default_board_state);
        assert_eq!(black_rooks, 9295429630892703744);

        let black_bishops = bitboard_from_char('b', &default_board_state);
        assert_eq!(black_bishops, 2594073385365405696);

        let black_knights = bitboard_from_char('n', &default_board_state);
        assert_eq!(black_knights, 4755801206503243776);

        let black_pawns = bitboard_from_char('p', &default_board_state);
        assert_eq!(black_pawns, 71776119061217280);
    }
}
