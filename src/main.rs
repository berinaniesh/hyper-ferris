mod constants;

fn init_attack_tables (){
    unsafe {
        for side in 0..2 {
            for square in 0..64 {
                constants::PAWN_ATTACKS[side][square] = set_pawn_attack_squares(side, square as u64);
                if side == 1 {
                    constants::KNIGHT_ATTACKS[square] = set_knight_attack_squares(square as u64);
                    constants::BISHOP_ATTACKS[square] = set_bishop_attack_squares(square as u64);
                    constants::ROOK_ATTACKS[square] = set_rook_attack_squares(square as u64);
                    constants::QUEEN_ATTACKS[square] = set_queen_attack_squares(square as u64);
                    constants::KING_ATTACKS[square] = set_king_attack_squares(square as u64);
                }    
            }
        }
    }

}

fn set_pawn_attack_squares (side: usize, square: u64) -> u64 {
    let mut attacks: u64 = 0;
    let mut bitboard: u64 = 0;
    set_bit(& mut bitboard, square);
    if side == constants::WHITE {
        if (constants::NOT_A_FILE & (bitboard >> 7)) > 0 {attacks |= bitboard >> 7};
        if (constants::NOT_H_FILE & (bitboard >> 9)) > 0 {attacks |= bitboard >> 9};
    }
    else if side == constants::BLACK {
        if (constants::NOT_A_FILE & (bitboard << 9)) > 0 {attacks |= bitboard << 9};
         if (constants::NOT_H_FILE & (bitboard << 7)) > 0 {attacks |= bitboard << 7};
    }
    return attacks;
}

fn set_knight_attack_squares (square: u64) -> u64 {
    let mut attacks: u64 = 0;
    let mut bitboard: u64 = 0;
    set_bit(&mut bitboard, square);
    if (constants::NOT_A_FILE & (bitboard >> 15)) > 0 {attacks |= bitboard >> 15};
    if (constants::NOT_H_FILE & (bitboard >> 17)) > 0 {attacks |= bitboard >> 17};
    if (constants::NOT_AB_FILE & (bitboard >> 6)) > 0 {attacks |= bitboard >> 6};
    if (constants::NOT_GH_FILE & (bitboard >> 10)) > 0 {attacks |= bitboard >> 10};
    if (constants::NOT_A_FILE & (bitboard << 17)) > 0 {attacks |= bitboard << 17};
    if (constants::NOT_H_FILE & (bitboard << 15)) > 0 {attacks |= bitboard << 15};
    if (constants::NOT_AB_FILE & (bitboard << 10)) > 0 {attacks |= bitboard << 10};
    if (constants::NOT_GH_FILE & (bitboard << 6)) > 0 {attacks |= bitboard << 6};
    return attacks;
}

fn set_bishop_attack_squares (square: u64) -> u64 {
    return 0;
}

fn set_rook_attack_squares (square: u64) -> u64 {
    return 0;
}

fn set_queen_attack_squares (square: u64) -> u64 {
    return 0;
}

fn set_king_attack_squares (square: u64) -> u64 {
    return 0;
}

fn get_bit(bitboard: u64, square: u64) -> bool {
    return if bitboard & (1 << square) > 0 {true} else {false};
}

fn set_bit(bitboard: &mut u64, square: u64) {
    *bitboard |= 1 << square;
}

fn pop_bit(bitboard: &mut u64, square: u64) {
    if get_bit(*bitboard, square) {*bitboard ^= 1 << square;} else {return};
}

fn print_bitboard (bitboard: u64) {
    for rank in 0..8 {
        print!("{}   ", 8-rank);
        for file in 0..8 {
            let square: u64 = rank*8 + file;
            print!("{} ", if get_bit(bitboard, square) {1} else {0});
        }
        println!();
    }
    println!("\n    a b c d e f g h\n");
    println!("  Bitboard: {}\n\n",bitboard);
}

fn main() {
    println!("\n\n   Hyper Ferris 0.1.0\n");
    init_attack_tables();
    unsafe {
    for square in 0..64 {
        print_bitboard(constants::KNIGHT_ATTACKS[square]);
    }   
}
}