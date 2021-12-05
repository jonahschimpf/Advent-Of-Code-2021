use std::fs;
fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let input: Vec<String> = contents
        .lines()
        .map(|x| x.to_string())
        .collect();

    let most_freq_string = most_freq_string(input.clone(), 0);
    
    let least_freq_string = least_freq_string(input.clone(), 0);

    let most_freq_int = convert_from_bin(most_freq_string);

    let least_freq_int = convert_from_bin(least_freq_string);

    let result : u64 = most_freq_int as u64 * least_freq_int as u64;
    println!("{}", result);
}

//Returns the string
fn most_freq_string(input:  Vec<String> , index: u32)-> String {
  if input.len() == 1 {
        return input[0].clone();
  } else {
        let len = input.len();
        let mut zero_count = 0;
        let mut one_count = 0;
        input.iter().for_each(|line| {
           if line.chars().nth(index as usize).unwrap() == '0' {
                zero_count += 1;            
           } else {
                one_count += 1;
           }
        });
        let filtered_most = filter_vec(input.clone(), if zero_count > one_count { '0' } else { '1' }, index);
        return most_freq_string(filtered_most, index + 1);
    }
    
}
fn least_freq_string(input:  Vec<String> , index: u32)-> String {
  if input.len() == 1 {
        return input[0].clone();
  } else {
        let len = input.len();
        let mut zero_count = 0;
        let mut one_count = 0;
        input.iter().for_each(|line| {
           if line.chars().nth(index as usize).unwrap() == '0' {
                zero_count += 1;            
           } else {
                one_count += 1;
           }
        });
        let filtered_least = filter_vec(input.clone(), if zero_count > one_count { '1' } else { '0' }, index);
        return least_freq_string(filtered_least, index + 1);
    }
    
}

fn filter_vec(input: Vec<String>, to_filter: char, index: u32) -> Vec<String> {
   let filtered: Vec<String> = input.into_iter().filter(|x| x.chars().nth(index as usize).unwrap() == to_filter).collect();
   return filtered;
}

fn convert_from_bin(bin_string: String) -> i32 {
    let mut total = 0;
    let base: i32 = 2;
    for (index, c) in bin_string.chars().enumerate() {
        if c == '1' {
           total += base.pow((bin_string.len() - index - 1) as u32);  
        }
    }
    return total;
}
