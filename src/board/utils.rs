pub fn only_one_bit_set(x: u64) -> bool {
    x != 0 && (x & (x - 1)) == 0
}
