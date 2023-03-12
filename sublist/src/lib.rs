#[derive(Debug, PartialEq, Eq)]
pub enum Comparison {
    Equal,
    Sublist,
    Superlist,
    Unequal,
}

/// Use of '.windows' which iter over all the array searching for the same vec as another
///
/// Doesnt work if the list has the same numbers but in diferent positions
///
///
pub fn sublist<T: PartialEq>(first_list: &[T], second_list: &[T]) -> Comparison {
    match (first_list.len(), second_list.len()) {
        (0, 0) => Comparison::Equal,
        (0, _) => Comparison::Sublist,
        (_, 0) => Comparison::Superlist,
        _ => match (first_list, second_list) {
            (x, y) if x == y => Comparison::Equal,
            (x, y) if x.windows(y.len()).any(|list| list == y) => Comparison::Superlist,
            (x, y) if y.windows(x.len()).any(|list| list == x) => Comparison::Sublist,
            (_, _) => Comparison::Unequal,
        },
    }
}
