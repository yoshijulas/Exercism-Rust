/// # Panics
///
/// this will not panic

pub fn is_armstrong_number(num: u32) -> bool {
    let num_digit = num.to_string();
    let power = num_digit.len();

    num_digit
        .chars()
        .map(|ch| u64::from(ch.to_digit(10).unwrap()))
        .map(|dig| dig.pow(power.try_into().unwrap()))
        .sum::<u64>()
        == u64::from(num)
}
