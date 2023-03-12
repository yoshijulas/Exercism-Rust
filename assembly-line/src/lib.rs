pub fn production_rate_per_hour(speed: u8) -> f64 {
    // Production rate per hour
    let production_rate = f64::from(speed);

    match speed {
        0 => 0.0,
        // 1 - 4 = 100%
        1..=4 => production_rate * 221.0,
        // 5 - 8 = 90%
        5..=8 => production_rate * 221.0 * 0.9,
        // 9 - 10 = 77%
        9..=10 => production_rate * 221.0 * 0.77,
        _ => unreachable!(),
    }
}

pub fn working_items_per_minute(speed: u8) -> u32 {
    (production_rate_per_hour(speed) / 60.0).floor() as u32
}