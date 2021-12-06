   
use std::fs;
use std::convert::TryInto;
fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let input: Vec<&str> = contents
        .lines()
        .collect();
    let num_of_lines = input.len();
    let length = input[0].len();
    let mut count_vec: Vec<i32> = vec![0; length];

    for line in input {
        for (index, letter) in line.chars().enumerate() {
            let bit = letter.to_digit(f64::RADIX).unwrap();
            if bit == 0 {
               count_vec[index] += 1; 
            } 
        }
    }

    // 1, 0, 1, 0, 0
    // 4, 3, 2, 1, 0
    let mut gamma_rate: u32 = 0;

    let mut epsilon_rate: u32  = 0;
    for (index, count) in count_vec.iter().enumerate() {
            let position: u32 = (length - index - 1) as u32;
            let base: i32 = 2;
            let add = base.pow(position as u32);
       if *count < (num_of_lines / 2) as i32 { //this should be a one
            gamma_rate += add as u32;
       } else {
           epsilon_rate += add as u32;
       }
    }


    println!("gammma rate is {}", gamma_rate);
    let result : u64 = epsilon_rate as u64 * gamma_rate as u64;
    println!("mult is {}", result);

    
}
