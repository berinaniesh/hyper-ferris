pub const a8: usize = 0;
pub const b8: usize = 1;
pub const c8: usize = 2;
pub const d8: usize = 3;
pub const e8: usize = 4;
pub const f8: usize = 5;
pub const g8: usize = 6;
pub const h8: usize = 7;
pub const a7: usize = 8;
pub const b7: usize = 9;
pub const c7: usize = 10;
pub const d7: usize = 11;
pub const e7: usize = 12;
pub const f7: usize = 13;
pub const g7: usize = 14;
pub const h7: usize = 15;
pub const a6: usize = 16;
pub const b6: usize = 17;
pub const c6: usize = 18;
pub const d6: usize = 19;
pub const e6: usize = 20;
pub const f6: usize = 21;
pub const g6: usize = 22;
pub const h6: usize = 23;
pub const a5: usize = 24;
pub const b5: usize = 25;
pub const c5: usize = 26;
pub const d5: usize = 27;
pub const e5: usize = 28;
pub const f5: usize = 29;
pub const g5: usize = 30;
pub const h5: usize = 31;
pub const a4: usize = 32;
pub const b4: usize = 33;
pub const c4: usize = 34;
pub const d4: usize = 35;
pub const e4: usize = 36;
pub const f4: usize = 37;
pub const g4: usize = 38;
pub const h4: usize = 39;
pub const a3: usize = 40;
pub const b3: usize = 41;
pub const c3: usize = 42;
pub const d3: usize = 43;
pub const e3: usize = 44;
pub const f3: usize = 45;
pub const g3: usize = 46;
pub const h3: usize = 47;
pub const a2: usize = 48;
pub const b2: usize = 49;
pub const c2: usize = 50;
pub const d2: usize = 51;
pub const e2: usize = 52;
pub const f2: usize = 53;
pub const g2: usize = 54;
pub const h2: usize = 55;
pub const a1: usize = 56;
pub const b1: usize = 57;
pub const c1: usize = 58;
pub const d1: usize = 59;
pub const e1: usize = 60;
pub const f1: usize = 61;
pub const g1: usize = 62;
pub const h1: usize = 63;

pub enum CellNames {
    A8, B8, C8, D8, E8, F8, G8, H8,
    A7, B7, C7, D7, E7, F7, G7, H7,
    A6, B6, C6, D6, E6, F6, G6, H6,
    A5, B5, C5, D5, E5, F5, G5, H5,
    A4, B4, C4, D4, E4, F4, G4, H4,
    A3, B3, C3, D3, E3, F3, G3, H3,
    A2, B2, C2, D2, E2, F2, G2, H2,
    A1, B1, C1, D1, E1, F1, G1, H1,
    }

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

pub const NOT_A_FILE: u64 = 18374403900871474942;
pub const NOT_AB_FILE: u64 = 18229723555195321596;
pub const NOT_HG_FILE: u64 = 4557430888798830399;
pub const NOT_H_FILE: u64 = 9187201950435737471;

pub static mut PAWN_ATTACKS:[[u64;64];2] = [[0;64];2];