
//some random number
static mut STATE: u32 = 1804289383;

fn get_random_u32_number () -> u32 {
    unsafe {
        let mut number: u32 = STATE;
        number ^= number << 13;
        number ^= number >> 17;
        number ^= number << 5;
        STATE = number;
        return number;
    }
}

fn get_random_u64_number () -> u64 {   
        let [n1, n2, n3, n4]: [u64;4];
        //slicing off everything more significant than 16 bits
        n1 = (get_random_u32_number() as u64) & 0xffff;
        n2 = (get_random_u32_number() as u64) & 0xffff;
        n3 = (get_random_u32_number() as u64) & 0xffff;
        n4 = (get_random_u32_number() as u64) & 0xffff;
        return n1 | (n2 << 16) | (n3 << 32) | (n4 << 48);
}

pub fn generate_magic_number () -> u64 {
    return get_random_u64_number() & get_random_u64_number() & get_random_u64_number();
}