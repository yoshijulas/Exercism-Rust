use std::{collections::HashMap, thread};

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    let mut new_hash = HashMap::new();
    let mut current_workers = 0;

    while worker_count < current_workers {
        thread::spawn(|| {
            new_hash.entry())
        })
    }

    new_hash
    // unimplemented!(
    //     "Count the frequency of letters in the given input '{:?}'. Ensure that you are using {} to process the input.",
    //     input,
    //     match worker_count {
    //         1 => "1 worker".to_string(),
    //         _ => format!("{worker_count} workers"),
    //     }
    // );
}
