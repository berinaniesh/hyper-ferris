pub fn get_bit (bitboard: u64, square: u64) -> bool {
    return if bitboard & (1 << square) > 0 {true} else {false};
}

pub fn set_bit (bitboard: &mut u64, square: u64) {
    *bitboard |= 1 << square;
}

pub fn pop_bit (bitboard: &mut u64, square: u64) {
    if get_bit(*bitboard, square) {*bitboard ^= 1 << square;} else {return};
}

pub fn pop_least_significant_bit (bitboard: &mut u64) {
    //Don't run it with bitboard = 0, thread panics due to underflow. 
        if *bitboard > 0 {*bitboard &= *bitboard - 1;}
}

pub fn count_no_of_set_bits (mut bitboard: u64) -> u8 {
    let mut count: u8 = 0;
    while bitboard > 0 {
        count = count + 1;
        bitboard &= bitboard -1;
    }
    return count;
}

pub fn find_least_significant_bit (bitboard: u64) -> usize {
    //Don't run this fn with bitboard = 0, gives a wrong answer.
    let mut count: usize = 0;
    let mut number: u64 = 1;
    if bitboard == 0 {return 0};
    while bitboard & number == 0 {
        count += 1;
        number = number << 1;
    }
    return count;
}
