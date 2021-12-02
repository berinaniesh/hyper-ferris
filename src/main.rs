mod constants;

fn magic_bishop_attacks (square: i64, blocker: u64) -> u64 {
    let mut attacks = 0;
    let tr = square/8;
    let tf = square%8;
    let mut rank;
    let mut file;

    rank = tr+1;
    file = tf+1;
    while rank <= 7 && file <= 7 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file)) & blocker) > 0 {break;}
        rank +=1;
        file +=1;
    }

    rank = tr - 1;
    file = tf + 1;
    while rank >= 0 && file <= 7 {
        attacks |= 1<<(rank *8 + file);
        if ((1<<(rank *8 + file))&blocker)>0 {break}
        rank -= 1;
        file += 1;
    }

    rank = tr + 1;
    file = tf - 1;
    while rank <= 7 && file >= 0 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file))&blocker)>0 {break}
        rank += 1;
        file -=1;
    }

    rank = tr-1;
    file = tf-1;
    while file >= 0 && rank >= 0 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file))&blocker)>0 {break}
        rank -=1;
        file -=1;
    }

    return attacks;
}

fn magic_rook_attacks (square: i64, blocker: u64) -> u64 {
    let mut attacks = 0;
    let tr = square/8;
    let tf = square%8;
    let mut rank;
    let mut file;

    rank = tr+1;
    file = tf;
    while rank <= 7 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file))&blocker)>0 {break}
        rank +=1;
    }

    rank = tr - 1;
    file = tf;
    while rank >= 0 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file))&blocker)>0 {break}
        rank -= 1;
    }

    rank = tr;
    file = tf - 1;
    while file >= 0 {
        attacks |= 1<<(rank*8 + file);
        if((1<<(rank*8 + file))&blocker)>0{break}
        file -=1;
    }

    rank = tr;
    file = tf + 1;
    while file <= 7 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file))&blocker)>0 {break}
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

fn pop_least_significant_bit(bitboard: &mut u64) {
        if *bitboard > 0 {*bitboard &= *bitboard - 1;}
}

fn count_no_of_set_bits (mut bitboard: u64) -> u8 {
    let mut count: u8 = 0;
    while bitboard > 0 {
        count = count + 1;
        bitboard &= bitboard -1;
    }
    return count;
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
    let mut bitboard: u64 = 0;
    set_bit(&mut bitboard, 44);
    set_bit(&mut bitboard, 22);
    set_bit(&mut bitboard, 13);
    println!("{}", count_no_of_set_bits(bitboard));
}