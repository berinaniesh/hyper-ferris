mod constants;

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
    let mut attacks: u64 = 0;
    let mut bitboard: u64 = 0;
    set_bit(&mut bitboard, square);
    attacks |= bitboard << 8;
    attacks |= bitboard >> 8;
    if ((bitboard << 1) & constants::NOT_A_FILE) > 0 {attacks |= bitboard << 1};
    if ((bitboard >> 1) & constants::NOT_H_FILE) > 0 {attacks |= bitboard >> 1};
    if ((bitboard << 7) & constants::NOT_H_FILE) > 0 {attacks |= bitboard << 7};
    if ((bitboard >> 7) & constants::NOT_A_FILE) > 0 {attacks |= bitboard >> 7};
    if ((bitboard << 9) & constants::NOT_A_FILE) > 0 {attacks |= bitboard << 9};
    if ((bitboard >> 9) & constants::NOT_H_FILE) > 0 {attacks |= bitboard >> 9};
    return attacks;
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
    for square in 0..64 {
        unsafe {
            constants::KING_ATTACKS[square] = set_king_attack_squares(square as u64);
            print_bitboard(constants::KING_ATTACKS[square]);
        }
    }
}