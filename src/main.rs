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

//=======================================================
const a8: i8 = 0;
const b8: i8 = 1;
const c8: i8 = 2;
const d8: i8 = 3;
const e8: i8 = 4;
const f8: i8 = 5;
const g8: i8 = 6;
const h8: i8 = 7;
const a7: i8 = 8;
const b7: i8 = 9;
const c7: i8 = 10;
const d7: i8 = 11;
const e7: i8 = 12;
const f7: i8 = 13;
const g7: i8 = 14;
const h7: i8 = 15;
const a6: i8 = 16;
const b6: i8 = 17;
const c6: i8 = 18;
const d6: i8 = 19;
const e6: i8 = 20;
const f6: i8 = 21;
const g6: i8 = 22;
const h6: i8 = 23;
const a5: i8 = 24;
const b5: i8 = 25;
const c5: i8 = 26;
const d5: i8 = 27;
const e5: i8 = 28;
const f5: i8 = 29;
const g5: i8 = 30;
const h5: i8 = 31;
const a4: i8 = 32;
const b4: i8 = 33;
const c4: i8 = 34;
const d4: i8 = 35;
const e4: i8 = 36;
const f4: i8 = 37;
const g4: i8 = 38;
const h4: i8 = 39;
const a3: i8 = 40;
const b3: i8 = 41;
const c3: i8 = 42;
const d3: i8 = 43;
const e3: i8 = 44;
const f3: i8 = 45;
const g3: i8 = 46;
const h3: i8 = 47;
const a2: i8 = 48;
const b2: i8 = 49;
const c2: i8 = 50;
const d2: i8 = 51;
const e2: i8 = 52;
const f2: i8 = 53;
const g2: i8 = 54;
const h2: i8 = 55;
const a1: i8 = 56;
const b1: i8 = 57;
const c1: i8 = 58;
const d1: i8 = 59;
const e1: i8 = 60;
const f1: i8 = 61;
const g1: i8 = 62;
const h1: i8 = 63;
//===============================================================
//Useful Constants

//Side to move constant
const WHITE: i8 = 0;
const BLACK: i8 = 1;

//Not specific file constants (for attack tables)
const NOT_A_FILE: u64 = 18374403900871474942;
const NOT_AB_FILE: u64 = 18229723555195321596;
const NOT_HG_FILE: u64 = 4557430888798830399;
const NOT_H_FILE: u64 = 9187201950435737471;

//Attack Tables generation

fn mask_attack_squares (bitboard: &mut u64, square: u64, side: i8) {
    let attack: u64 = 0;

    if side == 0 {
        //*bitboard 
    }
    else {

    }
}

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
    bitboard |= 1 << f4;
    print_bitboard(bitboard);
}