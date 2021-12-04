
pub fn magic_bishop_attacks (square: i64, blocker: u64) -> u64 {
    let mut attacks = 0;
    let tr = square/8;
    let tf = square%8;
    let mut rank;
    let mut file;

    rank = tr+1;
    file = tf+1;
    while rank <= 7 && file <= 7 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file)) & blocker) > 0 {break;}
        rank +=1;
        file +=1;
    }

    rank = tr - 1;
    file = tf + 1;
    while rank >= 0 && file <= 7 {
        attacks |= 1<<(rank *8 + file);
        if ((1<<(rank *8 + file))&blocker)>0 {break}
        rank -= 1;
        file += 1;
    }

    rank = tr + 1;
    file = tf - 1;
    while rank <= 7 && file >= 0 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file))&blocker)>0 {break}
        rank += 1;
        file -=1;
    }

    rank = tr-1;
    file = tf-1;
    while file >= 0 && rank >= 0 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file))&blocker)>0 {break}
        rank -=1;
        file -=1;
    }

    return attacks;
}

pub fn magic_rook_attacks (square: i64, blocker: u64) -> u64 {
    let mut attacks = 0;
    let tr = square/8;
    let tf = square%8;
    let mut rank;
    let mut file;

    rank = tr+1;
    file = tf;
    while rank <= 7 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file))&blocker)>0 {break}
        rank +=1;
    }

    rank = tr - 1;
    file = tf;
    while rank >= 0 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file))&blocker)>0 {break}
        rank -= 1;
    }

    rank = tr;
    file = tf - 1;
    while file >= 0 {
        attacks |= 1<<(rank*8 + file);
        if((1<<(rank*8 + file))&blocker)>0{break}
        file -=1;
    }

    rank = tr;
    file = tf + 1;
    while file <= 7 {
        attacks |= 1<<(rank*8 + file);
        if ((1<<(rank*8 + file))&blocker)>0 {break}
        file += 1;
    }

    return attacks;
}

pub fn set_rook_attack_squares (square: i64) -> u64 {
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

pub fn set_bishop_attack_squares (square: i64) -> u64 {
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