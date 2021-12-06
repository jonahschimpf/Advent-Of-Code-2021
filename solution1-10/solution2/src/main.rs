use std::fs;
use std::env;
fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let vector_of_nums: Vec<i32> = contents.lines().map(|x| x.parse::<i32>().unwrap()).collect();
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);
    let window_size: i32 = args[1].parse().unwrap();
    
    let vec_len = vector_of_nums.len();
    let mut increase = 0;
    for i in 0..vec_len {
        let copy_index = i as i32; 
        let first_window = get_win(copy_index, window_size, &vector_of_nums);
        let first_window_value;
        match first_window {
            None => break,
            Some(value) => {
                first_window_value = value;
            },
        }
        let second_window = get_win(copy_index + 1, window_size, &vector_of_nums);
        let second_window_value;
        match second_window {
            None => {
                println!("Breaking at index {}", i+1);
                break
            }
            Some(value) => {
                second_window_value = value;
            },
        }
        if second_window_value > first_window_value {
            increase +=1;
        }
    }
    println!("There are {}", increase);

}


fn get_win(index: i32, window_size: i32, nums: &Vec<i32>) -> Option<i32> {
   if index + window_size > nums.len() as i32 {
        return None;
   } else {
       let mut window = 0;
        for i in index ..index + window_size {
            window += nums[i as usize];
        }
        return Some(window);
   }
}
