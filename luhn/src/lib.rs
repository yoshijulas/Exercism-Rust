/// Check a Luhn checksum.

pub fn is_valid(code: &str) -> bool {
    let code = code.trim();
    if code.len() <= 1 {
        return false;
    }
    let mut sum = 0;

    let vec_char_code = code.chars().rev().filter(|ch| !ch.is_whitespace());
    // .collect::<Vec<_>>();

    for (pos, ch) in vec_char_code.enumerate() {
        if !ch.is_ascii_digit() {
            return false;
        }

        sum += match ch as i32 - 48 {
            val if pos % 2 != 0 => {
                if val > 4 {
                    val * 2 - 9
                } else {
                    val * 2
                }
            }
            val => val,
        }
    }

    sum % 10 == 0
}
