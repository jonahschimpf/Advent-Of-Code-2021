use std::fs;

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let input: Vec<&str> = contents
        .lines()
        .collect();

    let mut vert : i32 = 0;
    let mut horz : i32 = 0;
    let mut aim : i32 = 0;
    for line in input {
        let dir: Vec<&str> = line.split(" ").collect();
        if dir[0] == "forward" {
            let value = dir[1].parse::<i32>().unwrap();
            horz += value;
            vert += value * aim;
        }
        if dir[0] == "down" {
            aim  += dir[1].parse::<i32>().unwrap();
        }
        if dir[0] == "up" {
            aim  -= dir[1].parse::<i32>().unwrap();
            if vert < 0 {
                vert = 0;
            }
        }
    }

    
    println!("mult is {}", horz * vert);
}
