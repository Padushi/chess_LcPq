pub struct Piece {
    pub piece_type: char,
    pub piece_color: bool, // true for white, false for black

    pub coord: String,
    pub index: usize,

    pub is_orthogonal_slider: bool,
    pub is_diagonal_slider: bool,
    pub is_sliding_piece: bool,
}

pub fn build_piece_from_index(state: &Vec<char>, index: usize) -> Piece {
    use super::super::board::board_utils::coordinates_from_index;

    Piece {
        piece_type: state[index],
        piece_color: state[index].is_uppercase(),

        coord: coordinates_from_index(index),
        index: index,

        is_orthogonal_slider: matches!(state[index], 'r' | 'R' | 'Q' | 'q'),
        is_diagonal_slider: matches!(state[index], 'b' | 'B' | 'Q' | 'q'),
        is_sliding_piece: matches!(state[index], 'b' | 'B' | 'r' | 'R' | 'q' | 'Q'),
    }
}
