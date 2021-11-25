fn set_king_attack_squares (square: u64) -> u64 {
    let mut attacks: u64 = 0;
    let mut bitboard: u64 = 0;
    set_bit(&mut bitboard, square);
    attacks |= bitboard << 8;
    attacks |= bitboard >> 8;
    if ((bitboard << 1) & constants::NOT_A_FILE) > 0 {attacks |= bitboard << 1};
    if ((bitboard >> 1) & constants::NOT_H_FILE) > 0 {attacks |= bitboard >> 1};
    if ((bitboard << 7) & constants::NOT_H_FILE) > 0 {attacks |= bitboard << 7};
    if ((bitboard >> 7) & constants::NOT_A_FILE) > 0 {attacks |= bitboard >> 7};
    if ((bitboard << 9) & constants::NOT_A_FILE) > 0 {attacks |= bitboard << 9};
    if ((bitboard >> 9) & constants::NOT_H_FILE) > 0 {attacks |= bitboard >> 9};
    return attacks;
}

