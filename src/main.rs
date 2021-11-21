enum CellNames {
A8, B8, C8, D8, E8, F8, G8, H8,
A7, B7, C7, D7, E7, F7, G7, H7,
A6, B6, C6, D6, E6, F6, G6, H6,
A5, B5, C5, D5, E5, F5, G5, H5,
A4, B4, C4, D4, E4, F4, G4, H4,
A3, B3, C3, D3, E3, F3, G3, H3,
A2, B2, C2, D2, E2, F2, G2, H2,
A1, B1, C1, D1, E1, F1, G1, H1,
}

//Useful Constants

//Side to move constant
const WHITE: i8 = 0;
const BLACK: i8 = 1;

//Not specific file constants (for attack tables)
const NOT_A_FILE: u64 = 18374403900871474942;
const NOT_B_FILE: u64 = 18302063728033398269;
const NOT_G_FILE: u64 = 13816973012072644543;
const NOT_H_FILE: u64 = 9187201950435737471;




// Bit manipulation functions
//=============================================================================
fn get_bit(bitboard: u64, square: u64) -> bool {
    return if bitboard & (1 << square) > 0 {true} else {false};
}

fn set_bit(bitboard: &mut u64, square: u64) {
    *bitboard |= 1 << square;
}

fn pop_bit(bitboard: &mut u64, square: u64) {
    if get_bit(*bitboard, square) {*bitboard ^= 1 << square;} else {return};
}
//=============================================================================


fn print_bitboard (bitboard: u64) {
    for rank in 0..8 {
        print!("{}   ", 8-rank);
        for file in 0..8 {
            let square: u64 = rank*8 + file;
            //print!("{}\t",square);
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
    for rank in 0..8 {
        for file in 0..8 {
            let square = rank * 8 + file;
            if file !=1 {
                set_bit(&mut bitboard, square);
            }
        }
    }
    print_bitboard(bitboard);
}