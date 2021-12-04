
static mut state: u32 = 1804289383;

pub fn get_random_u32_number () -> u32 {
    unsafe {
        let mut number: u32 = state;
        number ^= number << 13;
        number ^= number >> 17;
        number ^= number << 5;
        state = number;
    return number;
    }
}

pub fn get_random_u64_number () -> u64 {
    unsafe{    
        let [n1, n2, n3, n4]: [u64;4];
        n1 = 6;
        return n1;
    }
}
