pub fn collatz(n: u64) -> Option<u64> {
    match n {
        (..=0) => None,
        1 => Some(0),
        mut n => {
            let mut tries: u64 = 0;
            while n != 1 {
                if n == u64::MAX {
                    return None;
                }
                if n % 2 == 0 {
                    n /= 2;
                    tries += 1;
                } else {
                    n = n.checked_mul(3)?.checked_add(1)? / 2;
                    tries += 2;
                }
            }
            Some(tries)
        }
    }
}
