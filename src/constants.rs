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

pub const SQUARES:[&str;64] = [
    "a8", "b8", "c8", "d8", "e8", "f8", "g8", "h8",
    "a7", "b7", "c7", "d7", "e7", "f7", "g7", "h7",
    "a6", "b6", "c6", "d6", "e6", "f6", "g6", "h6",
    "a5", "b5", "c5", "d5", "e5", "f5", "g5", "h5",
    "a4", "b4", "c4", "d4", "e4", "f4", "g4", "h4",
    "a3", "b3", "c3", "d3", "e3", "f3", "g3", "h3",
    "a2", "b2", "c2", "d2", "e2", "f2", "g2", "h2",
    "a1", "b1", "c1", "d1", "e1", "f1", "g1", "h1",
];

pub const WHITE: usize = 0;
pub const BLACK: usize = 1;

pub const NOT_A_FILE: u64 = 18374403900871474942;
pub const NOT_AB_FILE: u64 = 18229723555195321596;
pub const NOT_GH_FILE: u64 = 4557430888798830399;
pub const NOT_H_FILE: u64 = 9187201950435737471;


//attack tables
//============================================================

pub const BISHOP_RELEVANT_BITS: [u64;64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    6, 5, 5, 5, 5, 5, 5, 6, 
];

pub const ROOK_RELEVANT_BITS: [u64;64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12,
];

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
    18014398509481984,
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
    0,
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
    9077567998918656,
];
pub const KING_ATTACKS:[u64;64] = [
    770,
    1797,
    3594,
    7188,
    14376,
    28752,
    57504,
    49216,
    197123,
    460039,
    920078,
    1840156,
    3680312,
    7360624,
    14721248,
    12599488,
    50463488,
    117769984,
    235539968,
    471079936,
    942159872,
    1884319744,
    3768639488,
    3225468928,
    12918652928,
    30149115904,
    60298231808,
    120596463616,
    241192927232,
    482385854464,
    964771708928,
    825720045568,
    3307175149568,
    7718173671424,
    15436347342848,
    30872694685696,
    61745389371392,
    123490778742784,
    246981557485568,
    211384331665408,
    846636838289408,
    1975852459884544,
    3951704919769088,
    7903409839538176,
    15806819679076352,
    31613639358152704,
    63227278716305408,
    54114388906344448,
    216739030602088448,
    505818229730443264,
    1011636459460886528,
    2023272918921773056,
    4046545837843546112,
    8093091675687092224,
    16186183351374184448,
    13853283560024178688,
    144959613005987840,
    362258295026614272,
    724516590053228544,
    1449033180106457088,
    2898066360212914176,
    5796132720425828352,
    11592265440851656704,
    4665729213955833856,
];

pub const BISHOP_ATTACKS:[u64;64] = [
    18049651735527936,
    70506452091904,
    275415828992,
    1075975168,
    38021120,
    8657588224,
    2216338399232,
    567382630219776,
    9024825867763712,
    18049651735527424,
    70506452221952,
    275449643008,
    9733406720,
    2216342585344,
    567382630203392,
    1134765260406784,
    4512412933816832,
    9024825867633664,
    18049651768822272,
    70515108615168,
    2491752130560,
    567383701868544,
    1134765256220672,
    2269530512441344,
    2256206450263040,
    4512412900526080,
    9024834391117824,
    18051867805491712,
    637888545440768,
    1135039602493440,
    2269529440784384,
    4539058881568768,
    1128098963916800,
    2256197927833600,
    4514594912477184,
    9592139778506752,
    19184279556981248,
    2339762086609920,
    4538784537380864,
    9077569074761728,
    562958610993152,
    1125917221986304,
    2814792987328512,
    5629586008178688,
    11259172008099840,
    22518341868716544,
    9007336962655232,
    18014673925310464,
    2216338399232,
    4432676798464,
    11064376819712,
    22137335185408,
    44272556441600,
    87995357200384,
    35253226045952,
    70506452091904,
    567382630219776,
    1134765260406784,
    2832480465846272,
    5667157807464448,
    11333774449049600,
    22526811443298304,
    9024825867763712,
    18049651735527936,
];
pub const ROOK_ATTACKS:[u64;64] = [
    282578800148862,
    565157600297596,
    1130315200595066,
    2260630401190006,
    4521260802379886,
    9042521604759646,
    18085043209519166,
    36170086419038334,
    282578800180736,
    565157600328704,
    1130315200625152,
    2260630401218048,
    4521260802403840,
    9042521604775424,
    18085043209518592,
    36170086419037696,
    282578808340736,
    565157608292864,
    1130315208328192,
    2260630408398848,
    4521260808540160,
    9042521608822784,
    18085043209388032,
    36170086418907136,
    282580897300736,
    565159647117824,
    1130317180306432,
    2260632246683648,
    4521262379438080,
    9042522644946944,
    18085043175964672,
    36170086385483776,
    283115671060736,
    565681586307584,
    1130822006735872,
    2261102847592448,
    4521664529305600,
    9042787892731904,
    18085034619584512,
    36170077829103616,
    420017753620736,
    699298018886144,
    1260057572672512,
    2381576680245248,
    4624614895390720,
    9110691325681664,
    18082844186263552,
    36167887395782656,
    35466950888980736,
    34905104758997504,
    34344362452452352,
    33222877839362048,
    30979908613181440,
    26493970160820224,
    17522093256097792,
    35607136465616896,
    9079539427579068672,
    8935706818303361536,
    8792156787827803136,
    8505056726876686336,
    7930856604974452736,
    6782456361169985536,
    4485655873561051136,
    9115426935197958144,
];
pub static mut QUEEN_ATTACKS:[u64;64] = [0;64];