pub const fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    let quotient = dividend / divisor;
    let remainder = dividend % divisor;
    (quotient, remainder)
}

pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
    iter.enumerate().filter(|(x, _)| x % 2 == 0).map(|(_, y)| y)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub const fn manhattan(&self) -> i16 {
        (0 - self.0).abs() + (0 - self.1).abs()
    }
}
