use std::fs;

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    
    let mut prev = std::i32::MAX;
    let mut count = 0;
    let mut iter = contents.lines(); 
    while let Some(number_str) = iter.next() {
        println!("On {}", number_str);
        let curr_num: i32 = number_str.parse().unwrap();
        if curr_num > prev {
            count+=1;
        }
        prev = curr_num;
    }
    println!("The count is: {}", count);
}
