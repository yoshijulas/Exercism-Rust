pub fn factors(n: u64) -> Vec<u64> {
    let calculate_factors = |res| (2..=res).into_iter().find(|i| res % i == 0).unwrap();

    let mut res = n;
    let mut factors = Vec::new();

    while res != 1 {
        let factor_num = calculate_factors(res);
        factors.push(factor_num);
        res /= factor_num;
    }

    factors
}
