mod constants;

fn set_rook_attack_squares (square: i64) -> u64 {
    let mut attacks = 0;
    let tr = square/8;
    let tf = square%8;
    let mut rank;
    let mut file;

    rank = tr+1;
    file = tf;
    while rank < 7 {
        attacks |= 1<<(rank*8 + file);
        rank +=1;
    }

    rank = tr - 1;
    file = tf;
    while rank > 0 {
        attacks |= 1<<(rank*8 + file);
        rank -= 1;
    }

    rank = tr;
    file = tf - 1;
    while file > 0 {
        attacks |= 1<<(rank*8 + file);
        file -=1;
    }

    rank = tr;
    file = tf + 1;
    while file < 7 {
        attacks |= 1<<(rank*8 + file);
        file += 1;
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
    //print_bitboard(set_rook_attack_squares(constants::d4 as i64));
    unsafe {
        for square in 0..64 {
            constants::ROOK_ATTACKS[square] = set_rook_attack_squares(square as i64);
            print_bitboard(constants::ROOK_ATTACKS[square]);
        }
    }
}