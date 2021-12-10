use std::fs;
use std::collections::VecDeque;

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let input: Vec<i32> = contents
        .lines()
        .map(|x| check_err(x.to_string()))
        .collect();

    println!("The sum is {}", input.into_iter().sum::<i32>());

    
}

fn check_err(line: String) -> i32 {
    let mut stack: VecDeque<char> = VecDeque::new();   
    for c in line.chars() {
       if c == '(' || c == '{' || c == '[' || c == '<' {
            stack.push_front(c);
       } else {
            let open = stack.pop_front().unwrap();
            if !is_pair(&c, &open) {
                if c == ')' {
                    return 3;
                } else if c == ']' {
                    return 57;
                } else if c == '}' {
                    return 1197;
                } else if c == '>' {
                    return 25137;
                } else {
                    panic!("Killll me");
                }
            }
       }
    }
    return 0;
}

fn is_pair(close: &char, open: &char) -> bool{
    return *open == '(' && *close == ')' ||
    *open == '[' && *close == ']' ||
    *open == '{' && *close == '}' ||
    *open == '<' && *close == '>';
}
