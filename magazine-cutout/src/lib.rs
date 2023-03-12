// This stub file contains items that aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

use std::collections::HashMap;

pub fn can_construct_note(magazine: &[&str], note: &[&str]) -> bool {
    let mut counter = HashMap::new();

    for word in magazine {
        *counter.entry(word).or_insert(0) += 1;
    }
    for word in note {
        let num = counter.entry(word).or_insert(0);
        if *num <= 0 {
            return false;
        }
        *num -= 1;
    }

    true
}