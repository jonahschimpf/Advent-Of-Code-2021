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
    for line in input {
        let dir: Vec<&str> = line.split(" ").collect();
        if dir[0] == "forward" {
            horz += dir[1].parse::<i32>().unwrap();
        }
        if dir[0] == "down" {
            vert  += dir[1].parse::<i32>().unwrap();
        }
        if dir[0] == "up" {
            vert  -= dir[1].parse::<i32>().unwrap();
            if vert < 0 {
                vert = 0;
            }
        }
    }

    
    println!("mult is {}", horz * vert);
}
