use std::collections::HashMap;

pub fn solve(input: &str) -> Option<HashMap<char, u8>> {
    // unimplemented!("Solve the alphametic {:?}", input)
    // let mut answer = HashMap::new();
    let mut test = HashMap::new();

    let new_input = input
        .trim()
        .split(|c| c == '+' || c == '=')
        .filter_map(|ch| if ch.is_empty() { None } else { Some(ch.trim()) })
        .collect::<Vec<_>>();

    let charchar = new_input
        .iter()
        .map(|ch| ch.chars().collect())
        .collect::<Vec<Vec<_>>>();

    for i in &charchar {
        for j in i {
            test.entry(j).or_insert(0);
        }
    }

    //? Read last vector
    let last_in_vec = &charchar.len() - 1;

    //? bruteforce time :)

    let len_of_letters = test.len() - 1;



    // for i in 0..10 {
    //     for j in 0..10 {
    //         answer.entry(charchar[len_fvec][0]).or_insert(1);
    //     }
    // }

    println!("New -> {test:?}");

    //? another method -- ignore

    // let len_fvec = charchar.len() - 1;
    // println!("{len_fvec}");
    // println!("New -> {charchar:?}");

    // answer.entry(charchar[len_fvec][0]).or_insert(1);

    // if (charchar[len_fvec].len() - 1) == 0 {
    //     return None;
    // }

    // answer.entry(charchar[len_fvec][1]).or_insert(0);

    // println!("OLD -> {input:?}");
    // println!("size -> {}", charchar.len());
    // println!("size -> {}", charchar[charchar.len() - 1][0]);

    None
}
