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

fn init_attack_tables (){
    unsafe {
        for side in 0..2 {
            for square in 0..64 {
                constants::PAWN_ATTACKS[side][square] = set_pawn_attack_squares(side, square as u64);
                if side == 1 {
                    constants::KNIGHT_ATTACKS[square] = set_knight_attack_squares(square as u64);
                    constants::BISHOP_ATTACKS[square] = set_bishop_attack_squares(square as u64);
                    constants::ROOK_ATTACKS[square] = set_rook_attack_squares(square as u64);
                    constants::QUEEN_ATTACKS[square] = set_queen_attack_squares(square as u64);
                    constants::KING_ATTACKS[square] = set_king_attack_squares(square as u64);
                }    
            }
        }
    }

}

fn set_pawn_attack_squares (side: usize, square: u64) -> u64 {
    let mut attacks: u64 = 0;
    let mut bitboard: u64 = 0;
    set_bit(& mut bitboard, square);
    if side == constants::WHITE {
        if (constants::NOT_A_FILE & (bitboard >> 7)) > 0 {attacks |= bitboard >> 7};
        if (constants::NOT_H_FILE & (bitboard >> 9)) > 0 {attacks |= bitboard >> 9};
    }
    else if side == constants::BLACK {
        if (constants::NOT_A_FILE & (bitboard << 9)) > 0 {attacks |= bitboard << 9};
         if (constants::NOT_H_FILE & (bitboard << 7)) > 0 {attacks |= bitboard << 7};
    }
    return attacks;
}

fn set_knight_attack_squares (square: u64) -> u64 {
    return 0;
    let mut attacks: u64 = 0;
    let mut bitboard: u64 = 0;
    set_bit(&mut bitboard, square);
    if (constants::NOT_A_FILE & (bitboard >> 15)) > 0 {attacks |= bitboard >> 15};
    if (constants::NOT_H_FILE & (bitboard >> 17)) > 0 {attacks |= bitboard >> 17};
    if (constants::NOT_AB_FILE & (bitboard >> 6)) > 0 {attacks |= bitboard >> 6};
    if (constants::NOT_GH_FILE & (bitboard >> 10)) > 0 {attacks |= bitboard >> 10};
    if (constants::NOT_A_FILE & (bitboard << 17)) > 0 {attacks |= bitboard << 17};
    if (constants::NOT_H_FILE & (bitboard << 15)) > 0 {attacks |= bitboard << 15};
    if (constants::NOT_AB_FILE & (bitboard << 10)) > 0 {attacks |= bitboard << 10};
    if (constants::NOT_GH_FILE & (bitboard << 6)) > 0 {attacks |= bitboard << 6};
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
