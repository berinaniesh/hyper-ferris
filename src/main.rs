mod constants;
mod attack_tables;
mod bit_manipulation;
mod rng;

use {
    bit_manipulation::{get_bit, set_bit, pop_bit, pop_least_significant_bit, find_least_significant_bit},
    attack_tables::{set_bishop_attack_squares, set_rook_attack_squares, magic_bishop_attacks, magic_rook_attacks, set_occupancy},
    rng::{get_random_u32_number, get_random_u64_number},
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


fn main() {
    println!("\n\n   Hyper Ferris 0.1.0\n");
    print_bitboard(get_random_u32_number() as u64);
    print_bitboard(get_random_u32_number() as u64 & 0xffff);
}