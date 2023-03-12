use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    let mut answer: HashSet<&str> = HashSet::new();
    let word_lower = word.to_lowercase();
    let mut word_set = word_lower.chars().collect::<Vec<char>>();
    word_set.sort_unstable();

    for poss_words in possible_anagrams {
        let poss_word_lower = poss_words.to_lowercase();
        let mut poss_set = poss_word_lower.chars().collect::<Vec<char>>();
        poss_set.sort_unstable();

        // println!("{poss_words:?}");

        if word_set == poss_set && poss_word_lower != word_lower {
            answer.insert(poss_words);
        }
    }

    answer
}

// Dont over complicate things, the solution is always easier

// pub fn anagrams_for<'a>(word: &'a str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
//     let mut answer: HashSet<&str> = HashSet::new();
//     let word_set = word.to_lowercase().chars().collect::<HashSet<char>>();

//     for pos_words in possible_anagrams {
//         let poss_set = pos_words.chars().collect::<HashSet<char>>();

//         println!("{pos_words:?}");

//         let idk = pos_words.to_lowercase().chars().collect();

//         if word_set == poss_set && word_set != idk {
//             answer.insert(pos_words);
//         }
//     }

//     answer
// }
