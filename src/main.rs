mod constants;
mod attack_tables;
mod bit_manipulation;
mod rng;

use {
    bit_manipulation::{get_bit, set_bit, pop_bit, pop_least_significant_bit, find_least_significant_bit, count_no_of_set_bits},
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
    attack_tables::init_slider_attacks(constants::BISHOP);
    attack_tables::init_slider_attacks(constants::ROOK);
    let occupancy: u64 = 0;
    print_bitboard(attack_tables::get_bishop_attacks(constants::e4, occupancy));
}