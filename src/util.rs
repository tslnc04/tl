#[inline(never)]
pub fn is_ident(c: u8) -> bool {
    c.is_ascii_digit()
        || c.is_ascii_uppercase()
        || c.is_ascii_lowercase()
        || c == b'-'
        || c == b'_'
        || c == b':'
        || c == b'+'
        || c == b'/'
}
