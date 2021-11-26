mod constants;

fn set_bishop_attack_squares (square: u64) -> u64 {
    let mut attacks: u64 = constants::PAWN_ATTACKS[0][constants::d4];
    let mut bitboard: u64 = 0;
    let tr = square/8;
    let tf = square%8;
/*
    for (r = tr + 1, f = tf + 1; r <= 6 && f <= 6; r++, f++) attacks |= (1ULL << (r * 8 + f));
    for (r = tr - 1, f = tf + 1; r >= 1 && f <= 6; r--, f++) attacks |= (1ULL << (r * 8 + f));
    for (r = tr + 1, f = tf - 1; r <= 6 && f >= 1; r++, f--) attacks |= (1ULL << (r * 8 + f));
    for (r = tr - 1, f = tf - 1; r >= 1 && f >= 1; r--, f--) attacks |= (1ULL << (r * 8 + f));

    let (x, y) = (1, 3);
    for (a, b) in (x+1..=6).zip(y+1..=6) {
        println!("{} {}", a, b);
    }

*/


    
    let mut rank = tr+1;
    let mut file = tf+1;
    let one:u64 = 1;
    while (rank < 7 && file < 7){
        attacks |= 1 << (rank*8 + file);
        rank++;
        file++;
    }
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
    /*unsafe {
        for square in 0..64 {
            constants::BISHOP_ATTACKS[square] = set_bishop_attack_squares (square as u64);
            print_bitboard(constants::BISHOP_ATTACKS[square]);
        }
    }*/
    print_bitboard(set_bishop_attack_squares(constants::d4 as u64));
}