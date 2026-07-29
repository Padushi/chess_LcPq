mod core;

fn main() {
    use core::board::board_core::build_board_from_fen;

    let default_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";
    let default_board = build_board_from_fen(default_fen);

    core::board::board_utils::print_board(&default_board, false, true);
}
