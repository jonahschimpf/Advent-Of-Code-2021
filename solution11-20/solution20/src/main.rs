use std::fs;
use std::collections::VecDeque;

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    
    let mut scores: Vec<u64> = Vec::new();
    let input: Vec<i32> = contents
        .lines()
        .map(|x| check_err(x.to_string(), &mut scores))
        .collect();

    println!("The sum is {}", input.into_iter().sum::<i32>());
    scores.sort();
    println!("middle is {}", scores[scores.len() / 2]);
}

fn check_err(line: String, scores: &mut Vec<u64>) -> i32 {
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
    //if it gets here its ok but maybe incomplete
    if !stack.is_empty() {
        let mut score = 0;
       while !stack.is_empty() {
           let c = stack.pop_front().unwrap(); 
           if c == '(' {
               score = score * 5 + 1; 
           } else if c == '[' {
                
               score = score * 5 + 2; 
           } else if c == '{' {
                
               score = score * 5 + 3; 
           } else if c== '<' {
                
               score = score * 5 + 4 
           } else {
                panic!("MEEK");
           }
       }
       scores.push(score);
    }
    return 0;
}

fn is_pair(close: &char, open: &char) -> bool{
    return *open == '(' && *close == ')' ||
    *open == '[' && *close == ']' ||
    *open == '{' && *close == '}' ||
    *open == '<' && *close == '>';
}
