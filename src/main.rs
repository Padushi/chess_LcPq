mod core;

fn main() {
    use core::board::bitboards::print_bitboard;
    use core::board::board_core::build_board_from_fen;

    let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let default_board = build_board_from_fen(default_fen);

    core::board::board_utils::print_board(&default_board, true);
    let white_pawns = core::board::bitboards::bitboard_from_char('p', &default_board.state);
    println!("{:#b}", white_pawns);

    print_bitboard(default_board.white_occupancy())
}
