
pub fn is_start_of_codepoint(byte: u8) -> bool {
    byte.leading_ones() == 0 || byte.leading_ones() > 1
}
