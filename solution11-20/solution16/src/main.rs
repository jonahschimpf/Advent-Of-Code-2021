use std::fs;
use std::collections::HashSet;

use std::collections::HashMap;
fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let mut signal_per_line : Vec<String> = Vec::new();

    let mut out_per_line : Vec<String> = Vec::new();
    contents.lines().for_each(|x| {
      let mut split = x.split(" | ");
    signal_per_line.push(split.next().unwrap().to_string());
    out_per_line.push(split.next().unwrap().to_string());
    });
    
    let vec_input: Vec<Vec<String>> = out_per_line.iter_mut().map(|x| x.split(" ").map(|x| x.to_string()).collect()).collect();
    
    let vec_signal: Vec<Vec<String>> = signal_per_line.iter_mut().map(|x| x.split(" ").map(|x| x.to_string()).collect()).collect(); //each vec in this list represents a line on the side before the "|"

   let mut sum = 0; 
    for i in 0..vec_signal.len() {
        let index = i as usize;
       sum += get_num(&vec_input[index], &vec_signal[index]); 
    }
    println!("sum {}", sum);
}


fn get_num(to_decode: &Vec<String>, from_signals: &Vec<String>) -> i32 {
   let mut letter_to_pattern: HashMap<i32, HashSet<char>> = HashMap::new(); 
   let mut patterns_len_5 : Vec<HashSet<char>> = Vec::new(); 

   let mut patterns_len_6 : Vec<HashSet<char>> = Vec::new(); 
   for s in from_signals.iter() {
        if s.len() == 2 {
            add_pattern(1, (*s).clone(), &mut letter_to_pattern);
        } else if s.len() == 3 {
            add_pattern(7, (*s).clone(), &mut letter_to_pattern);
        } else if s.len() == 4 { 
            add_pattern(4, (*s).clone(), &mut letter_to_pattern);
        } else if s.len() == 7 { 
            add_pattern(8, (*s).clone(), &mut letter_to_pattern);
        } else if s.len() == 5 {
            patterns_len_5.push(s.chars().collect());
        } else if s.len() == 6 { 
            patterns_len_6.push(s.chars().collect());
        }
   }
   //find what pattern 6 is
    let mut f_signal: char = '0';
    let mut c_signal: char = '0';
    let mut to_remove: usize = 0;
   for (index, pattern) in patterns_len_6.iter().enumerate() {
        if !letter_to_pattern.get(&1).unwrap().is_subset(&pattern) {
            println!("Something something {:?}", pattern);
            add_pattern_set(6, pattern.clone(), &mut letter_to_pattern); 
            for letter in letter_to_pattern.get(&1).unwrap().into_iter() {
                if pattern.contains(&letter) {
                    f_signal = letter.clone();
                } else {
                    c_signal = letter.clone();
                }
            }
            to_remove = index;;
        }
   }
   patterns_len_6.remove(to_remove);
   for (index, pattern) in patterns_len_5.iter().enumerate() {
       if letter_to_pattern.get(&1).unwrap().is_subset(&pattern) {
           add_pattern_set(3, pattern.clone(), &mut letter_to_pattern);
           to_remove = index;
       }
   }
   patterns_len_5.remove(to_remove);

   for pattern in patterns_len_6.iter() {
        if letter_to_pattern.get(&4).unwrap().is_subset(&pattern) {
            add_pattern_set(9, pattern.clone(), &mut letter_to_pattern);
        } else {
            add_pattern_set(0, pattern.clone(), &mut letter_to_pattern);
        }
   }


   for pattern in patterns_len_5.iter() {
        if pattern.contains(&f_signal) {
           add_pattern_set(5, pattern.clone(), &mut letter_to_pattern); 
        }
        if pattern.contains(&c_signal) { 
           add_pattern_set(2, pattern.clone(), &mut letter_to_pattern); 
        }
   }
   for (key, value) in &letter_to_pattern {
        println!("key {} value {:?}", key, value);
   }
   let mut ret = "".to_string();
   for string_num in to_decode.iter() {
        let converted = decode(string_num.clone(), &mut letter_to_pattern);
        ret.push_str(&converted);
   }
   println!("eek! {}", ret);
   return ret.parse().unwrap();
}

//decodes in i32
fn decode(s: String, letter_to_pattern: &mut HashMap<i32, HashSet<char>>) -> String{
    let set: HashSet<char> = s.chars().collect();
    for (key, pattern) in letter_to_pattern {
       if set == *pattern {
           return key.to_string();
       }
    }
    println!("ahhhh {}", s);
    return "FUCK".to_string();
}

fn add_pattern(key: i32, s: String, letter_to_pattern: &mut HashMap<i32, HashSet<char>>) {
    letter_to_pattern.insert(key, s.chars().collect()); 
}


fn add_pattern_set(key: i32, s: HashSet<char>, letter_to_pattern: &mut HashMap<i32, HashSet<char>>) {
    letter_to_pattern.insert(key, s); 
}
