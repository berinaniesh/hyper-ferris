mod constants;
mod attack_tables;
mod bit_manipulation;
mod rng;

use {
    bit_manipulation::{get_bit, set_bit, pop_bit, pop_least_significant_bit, find_least_significant_bit, count_no_of_set_bits},
    attack_tables::{set_bishop_attack_squares, set_rook_attack_squares, magic_bishop_attacks, magic_rook_attacks, set_occupancy},
    rng::{generate_magic_number},
};


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
        let product = attack_mask * magic_number;
        if count_no_of_set_bits(product as u64 & 0xff00000000000000) < 6 {continue}

        for idx in 0..4096 {
            used_attacks[idx] = 0;
        }
        
        let mut index: i32 = 0;
        let mut fail: i32 = 0;
         
        while fail !=0 && index < occupancy_indices as i32 {
            let magic_index: usize = ((occupancies[index as usize] * magic_number) >> (64 - relevant_bits)) as usize;
            if used_attacks[magic_index] == 0 {
                used_attacks[magic_index] = attacks[index as usize];
            }
            else if used_attacks[magic_index] != attacks[index as usize] {
                fail = 1;
            }
            if fail == 0 {
                return magic_number;
            }
            index += 1;
        }

    }
    println! ("Magic number fails");
    return 0;
}

fn init_magic_numbers () {
    for square in 0..64 {
        println!("{:#x}", find_magic_number(square, constants::ROOK_RELEVANT_BITS[square as usize], constants::ROOK))
    }
}

fn main() {
    println!("\n\n   Hyper Ferris 0.1.0\n");
    init_magic_numbers();
}