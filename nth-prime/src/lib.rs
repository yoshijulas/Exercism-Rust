/// # Panics
///
/// It Will Not Panic

pub fn nth(n: u64) -> u64 {
    let is_prime = |num: u64| (2..=(num as f64).sqrt() as u64).all(|i| num % i != 0);
    (2..).filter(|&x| is_prime(x)).nth(n as usize).unwrap()
}
