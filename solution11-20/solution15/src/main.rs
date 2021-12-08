use std::fs;

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut signal_per_line : Vec<String> = Vec::new();

    let mut out_per_line : Vec<String> = Vec::new();
    contents.lines().for_each(|x| {
      let mut split = x.split("|");
    signal_per_line.push(split.next().unwrap().to_string());
    out_per_line.push(split.next().unwrap().to_string());
    });
    
    let vec_input: Vec<Vec<String>> = out_per_line.iter_mut().map(|x| x.split(" ").map(|x| x.to_string()).collect()).collect();

    let mut count = 0;
    for list in vec_input.iter() {
        for string in list {
            let len = string.len();
            if len == 2 || len == 4 || len == 3 || len == 7 {
                count += 1;
            }
        }
    }
    println!("Count {}", count);
}
