struct Board {
    struct pieces_on_board {
        white_pawns: u64;
        black_pawns: u64;
        white_knights: u64;
        black_knights: u64;
        white_bishops: u64;
        black_bishops: u64;
        white_rooks: u64;
        black_rooks:u64;
        white_queens: u64;
        black_queens: u64;
        white_king: u64;
        black_king: u64;
    }
    castling_rights: u8;
    enpassant_squares: u64;


}
impl Board {
    pub fn new() -> Self {
        
    }

    fn reset board () {
        
    }
    fn set_en_passant_squares() {

    }
}

impl Board.pieces_on_board {
    fn set_bishops() {
        
    }
} 