pub fn brackets_are_balanced(string: &str) -> bool {
    let mut stack: Vec<char> = vec![];

    for char in string.chars() {
        match char {
            '[' => stack.push(']'),
            '(' => stack.push(')'),
            '{' => stack.push('}'),
            ']' | ')' | '}' => {
                if stack.pop() != Some(char) {
                    return false;
                }
            }
            _ => (),
        }
    }

    stack.is_empty()
}
