pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack = vec![];
    let brackets = string
        .chars()
        .filter(|ch| matches!(ch, '[' | '{' | '(' | ']' | '}' | ')' | '<' | '>'))
        .all(|c| match c {
            '(' | '{' | '[' | '<' => {
                stack.push(c);
                true
            }
            ')' => stack.pop() == Some('('),
            '}' => stack.pop() == Some('{'),
            ']' => stack.pop() == Some('['),
            '>' => stack.pop() == Some('<'),
            _ => true,
        });

    brackets && stack.is_empty()
}
