pub fn raindrops(n: u32) -> String {
    let mut answer = String::new();
    if n % 3 == 0 { answer.push_str("Pling"); }
    if n % 5 == 0 { answer.push_str("Plang"); }
    if n % 7 == 0 { answer.push_str("Plong"); }

    if answer.is_empty() {
        n.to_string()
    } else {
        answer
    }
}