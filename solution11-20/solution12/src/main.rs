//Something smells fishy...
use std::collections::HashMap;
use std::fs;
#[derive(Hash, Eq, PartialEq, Debug)]
struct Fishy {
    timer: i32,
    should_spawn: bool,
}


fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let input: Vec<&str> = contents
        .lines()
        .collect();

    let  list_o_fish: Vec<Fishy> = input[0].to_string().split(",").into_iter().map(|x| Fishy {timer: x.parse().unwrap(), should_spawn: false}).collect();

    let mut timer_map: HashMap<i32, u64> = HashMap::new();
    for fish in list_o_fish {
       if timer_map.contains_key(&fish.timer) {
            timer_map.insert(fish.timer, timer_map.get(&fish.timer).unwrap() + 1);
       } else {
            timer_map.insert(fish.timer, 1);
       }
    }

    let days = 256;

    for _day in 0..days {
        let mut to_add: Vec<u64> = vec![0; 9];  //index = timer, value= to change in next day
        for (timer, fish_count) in timer_map.iter_mut() {
           if *timer > 0 {
                let new_timer = timer - 1;
                to_add[new_timer as usize] = to_add[new_timer as usize] + *fish_count; //so the fish at new_timer should increase by fish count on the old timer
           } else {
                to_add[8] = *fish_count;
                to_add[6] += *fish_count; // += since the 7s will also add to it
           }
       }
       for i in 0..9 {
            let fish_count = to_add[i];
            timer_map.insert(i as i32, fish_count);
       }
    }
    let mut count = 0;

    let _value = timer_map.values().for_each(|x| count += x);
    println!("There are {} fishies", count)



    
}
