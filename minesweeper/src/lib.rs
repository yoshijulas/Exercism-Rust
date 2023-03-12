pub fn annotate(minefield: &[&str]) -> Vec<String> {
    let surrounding_cells = [
        (-1, -1),
        (-1, 0),
        (-1, 1),
        (0, -1),
        (0, 1),
        (1, -1),
        (1, 0),
        (1, 1),
    ];

    let mut vector_input = minefield
        .iter()
        .map(|ch| ch.as_bytes().to_vec())
        .collect::<Vec<_>>();

    let r = vector_input.len();
    if r == 0 {
        return vec![];
    }
    let c = vector_input[0].len();

    for pos_row in 0..r {
        for pos_col in 0..c {
            let mut mine = 0;
            if vector_input[pos_row][pos_col] != b'*' {
                match surrounding_cells.iter() {

          if pos_row > 0 && vector_input[pos_row - 1][pos_col] == b'*' {
                    mine += 1;
                }
                if pos_col > 0 && vector_input[pos_row][pos_col - 1] == b'*' {
                    mine += 1;
                }

                if pos_row + 1 < r && vector_input[pos_row + 1][pos_col] == b'*' {
                    mine += 1;
                }
                if pos_col + 1 < c && vector_input[pos_row][pos_col + 1] == b'*' {
                    mine += 1;
                }
                if pos_row > 0 && pos_col > 0 && vector_input[pos_row - 1][pos_col - 1] == b'*' {
                    mine += 1;
                }
                if pos_row + 1 < r && pos_col > 0 && vector_input[pos_row + 1][pos_col - 1] == b'*'
                {
                    mine += 1;
                }

                if pos_row + 1 < r
                    && pos_col + 1 < c
                    && vector_input[pos_row + 1][pos_col + 1] == b'*'
                {
                    mine += 1;
                }
                if pos_row > 0 && pos_col + 1 < c && vector_input[pos_row - 1][pos_col + 1] == b'*'
                {
                    mine += 1;
                }}

                if mine > 0 {
                    vector_input[pos_row][pos_col] = mine;
                } else {
                    vector_input[pos_row][pos_col] = b' ';
                }
            }
        }
    }

    let answer = vector_input
        .iter()
        .map(|x| {
            x.iter()
                .map(|&ch| {
                    if ch == b'*' || ch == b' ' {
                        ch as char
                    } else {
                        (ch + 48) as char
                    }
                })
                .collect::<String>()
        })
        .collect::<Vec<String>>();

    answer
}
