use {
    crate::bit_manipulation::{pop_bit, find_least_significant_bit, count_no_of_set_bits},
    crate::constants,
    crate::constants::{BISHOP_ATTACKS, ROOK_ATTACKS, BISHOP_MASKS, ROOK_MASKS, BISHOP_MAGIC_NUMBERS, ROOK_MAGIC_NUMBERS, BISHOP_RELEVANT_BITS, ROOK_RELEVANT_BITS}, 
    crate::rng::generate_magic_number,
};


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

fn set_bishop_attack_squares (square: i64) -> u64 {
    let mut attacks = 0;
    let tr = square/8;
    let tf = square%8;
    let mut rank;
    let mut file;

    rank = tr+1;
    file = tf+1;
    while rank < 7 && file < 7 {
        attacks |= 1<<(rank*8 + file);
        rank +=1;
        file +=1;
    }

    rank = tr - 1;
    file = tf + 1;
    while rank > 0 && file < 7 {
        attacks |= 1<<(rank *8 + file);
        rank -= 1;
        file += 1;
    }

    rank = tr + 1;
    file = tf - 1;
    while rank < 7 && file > 0 {
        attacks |= 1<<(rank*8 + file);
        rank += 1;
        file -=1;
    }

    rank = tr-1;
    file = tf-1;
    while file > 0 && rank > 0 {
        attacks |= 1<<(rank*8 + file);
        rank -=1;
        file -=1;
    }

    return attacks;
}

fn set_occupancy (index: i32, bits_in_mask: i32, mut attack_mask: u64) -> u64 {
    let mut occupancy = 0u64;
    for count in 0..bits_in_mask {
        let square = find_least_significant_bit (attack_mask);
        pop_bit (&mut attack_mask, square as u64);
        if (index & (1<<count)) > 0 {
            occupancy |= 1 << square;
        }
    }
    return occupancy;
}

fn find_magic_number (square: i64, relevant_bits: i32, piece: usize) -> u64 {
    let mut occupancies: [u64;4096] = [0;4096];
    let mut attacks: [u64;4096] = [0;4096];
    let mut used_attacks: [u64; 4096] = [0;4096];
    let attack_mask = if piece == constants::BISHOP {set_bishop_attack_squares(square)} else {set_rook_attack_squares(square)};
    let occupancy_indices:usize = 1 << relevant_bits;

    for index in 0..occupancy_indices {
        occupancies[index] = set_occupancy(index as i32, relevant_bits, attack_mask);
        attacks[index] = if piece == constants::BISHOP {magic_bishop_attacks(square, occupancies[index])} else {magic_rook_attacks(square, occupancies[index])};
    }

    for _ in 0..100000000 {
        let magic_number: u64 = generate_magic_number();
        let product = attack_mask.wrapping_mul(magic_number);
        // println!("Product is {}", product);
        if count_no_of_set_bits(product as u64 & 0xff00000000000000) < 6 {continue}

        for idx in 0..4096 {
            used_attacks[idx] = 0;
        }
        
        let mut index: i32 = 0;
        let mut success: bool = true;
         
        while success && index < occupancy_indices as i32 {
            let magic_index: usize = ((occupancies[index as usize] * magic_number) >> (64 - relevant_bits)) as usize;
            if used_attacks[magic_index] == 0 {
                used_attacks[magic_index] = attacks[index as usize];
            }
            else if used_attacks[magic_index] != attacks[index as usize] {
                success = false;
            }
            if success {
                return magic_number;
            }
            index += 1;
        }

    }
    println! ("Magic number fails");
    return 0;
}

pub fn init_magic_numbers () {
    for square in 0..64 {
        println!("{},", find_magic_number(square, constants::ROOK_RELEVANT_BITS[square as usize], constants::ROOK))
    }
    println!("\n\n\n");
    for square in 0..64 {
        println!("{},", find_magic_number(square, constants::BISHOP_RELEVANT_BITS[square as usize], constants::BISHOP))
    }
}



pub fn init_slider_attacks(piece: usize) {
    unsafe {
        for square in 0..64 {
            BISHOP_MASKS[square] = set_bishop_attack_squares(square as i64);
            ROOK_MASKS[square] = set_rook_attack_squares(square as i64);

            let attack_mask: u64 = if piece == constants::BISHOP {BISHOP_MASKS[square]} else {ROOK_MASKS[square]};
            let relevant_bits_count: u8 = count_no_of_set_bits(attack_mask);

            let occupancy_indices: i32 = 1 << relevant_bits_count;

            for index in 0..occupancy_indices {
                if piece == constants::BISHOP {
                    let occupancy: u64 = set_occupancy(index, relevant_bits_count as i32, attack_mask);
                    let magic_index: u64 = (occupancy.wrapping_mul(BISHOP_MAGIC_NUMBERS[square])) >> (64 -BISHOP_RELEVANT_BITS[square]);
                    BISHOP_ATTACKS[square][magic_index as usize] = magic_bishop_attacks(square as i64, occupancy);
                }
                else {
                    let occupancy: u64 = set_occupancy(index, relevant_bits_count as i32, attack_mask);
                    let magic_index: u64 = (occupancy.wrapping_mul(ROOK_MAGIC_NUMBERS[square])) >> (64 -ROOK_RELEVANT_BITS[square]);
                    ROOK_ATTACKS[square][magic_index as usize] = magic_rook_attacks(square as i64, occupancy);
                }
            }
        }
    }
}

pub fn get_bishop_attacks(square: usize, mut occupancy: u64) -> u64 {
    unsafe {
        occupancy &= BISHOP_MASKS[square];
        occupancy = occupancy.wrapping_mul(BISHOP_MAGIC_NUMBERS[square]);
        occupancy >>= 64 - BISHOP_RELEVANT_BITS[square];
        return BISHOP_ATTACKS[square][occupancy as usize];
    }
}

pub fn get_rook_attacks(square: usize, mut occupancy: u64) -> u64 {
    unsafe {
        occupancy &= ROOK_MASKS[square];
        occupancy = occupancy.wrapping_mul(ROOK_MAGIC_NUMBERS[square]);
        occupancy >>= 64 - ROOK_RELEVANT_BITS[square];
        return ROOK_ATTACKS[square][occupancy as usize];
    }
}