//board constants
//============================================================
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
pub const NOT_GH_FILE: u64 = 4557430888798830399;
pub const NOT_H_FILE: u64 = 9187201950435737471;


//attack tables
//============================================================
pub const PAWN_ATTACKS:[[u64;64];2] = [
[
0,
0,
0,
0,
0,
0,
0,
0,
2,
5,
10,
20,
40,
80,
160,
64,
512,
1280,
2560,
5120,
10240,
20480,
40960,
16384,
131072,
327680,
655360,
1310720,
2621440,
5242880,
10485760,
4194304,
33554432,
83886080,
167772160,
335544320,
671088640,
1342177280,
2684354560,
1073741824,
8589934592,
21474836480,
42949672960,
85899345920,
171798691840,
343597383680,
687194767360,
274877906944,
2199023255552,
5497558138880,
10995116277760,
21990232555520,
43980465111040,
87960930222080,
175921860444160,
70368744177664,
562949953421312,
1407374883553280,
2814749767106560,
5629499534213120,
11258999068426240,
22517998136852480,
45035996273704960,
18014398509481984
],
[
512,
1280,
2560,
5120,
10240,
20480,
40960,
16384,
131072,
327680,
655360,
1310720,
2621440,
5242880,
10485760,
4194304,
33554432,
83886080,
167772160,
335544320,
671088640,
1342177280,
2684354560,
1073741824,
8589934592,
21474836480,
42949672960,
85899345920,
171798691840,
343597383680,
687194767360,
274877906944,
2199023255552,
5497558138880,
10995116277760,
21990232555520,
43980465111040,
87960930222080,
175921860444160,
70368744177664,
562949953421312,
1407374883553280,
2814749767106560,
5629499534213120,
11258999068426240,
22517998136852480,
45035996273704960,
18014398509481984,
144115188075855872,
360287970189639680,
720575940379279360,
1441151880758558720,
2882303761517117440,
5764607523034234880,
11529215046068469760,
4611686018427387904,
0,
0,
0,
0,
0,
0,
0,
0
]
];
pub const KNIGHT_ATTACKS:[u64;64] = [
132096,
329728,
659712,
1319424,
2638848,
5277696,
10489856,
4202496,
33816580,
84410376,
168886289,
337772578,
675545156,
1351090312,
2685403152,
1075839008,
8657044482,
21609056261,
43234889994,
86469779988,
172939559976,
345879119952,
687463207072,
275414786112,
2216203387392,
5531918402816,
11068131838464,
22136263676928,
44272527353856,
88545054707712,
175990581010432,
70506185244672,
567348067172352,
1416171111120896,
2833441750646784,
5666883501293568,
11333767002587136,
22667534005174272,
45053588738670592,
18049583422636032,
145241105196122112,
362539804446949376,
725361088165576704,
1450722176331153408,
2901444352662306816,
5802888705324613632,
11533718717099671552,
4620693356194824192,
288234782788157440,
576469569871282176,
1224997833292120064,
2449995666584240128,
4899991333168480256,
9799982666336960512,
1152939783987658752,
2305878468463689728,
1128098930098176,
2257297371824128,
4796069720358912,
9592139440717824,
19184278881435648,
38368557762871296,
4679521487814656,
9077567998918656
];
pub static mut BISHOP_ATTACKS:[u64;64] = [0;64];
pub static mut ROOK_ATTACKS:[u64;64] = [0;64];
pub static mut QUEEN_ATTACKS:[u64;64] = [0;64];
pub static mut KING_ATTACKS:[u64;64] = [0;64];