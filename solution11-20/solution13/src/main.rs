//Something smells fishy...
use std::collections::HashMap;
use std::fs;

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    let input: Vec<i32> = contents.lines().next().unwrap().split(",").map(|x| x.parse().unwrap()).collect();

    let mut position_to_count: HashMap<i32, i32> = HashMap::new();
    
    let mut max_pos = 0;
    input.iter().for_each(|x| {
        if *x > max_pos {
            max_pos = *x;
        }
        if position_to_count.contains_key(x) {
          let old = *(position_to_count.get_mut(&x).unwrap());
          position_to_count.insert(*x, old + 1);
        } else {
            position_to_count.insert(*x, 1);
        }
    });

    for pos in 0..max_pos + 1 {
       if !position_to_count.contains_key(&pos) {
            position_to_count.insert(pos, 0);
       }
    }
    
    //let mut position_to_list_cost: HashMap<i32, Vec<i32>> = HashMap::new();
    let mut max_cost = 0;
    let mut min_cost = i32::MAX;

    let mut pos_to_fuel: HashMap<i32, Vec<i32>> = HashMap::new();
    for pos in 0..max_pos + 1 {
       let count = *position_to_count.get(&pos).unwrap();
       let fuel_list_for_pos: Vec<i32> = get_fuel_list_for_pos(pos, max_pos, count);
       pos_to_fuel.insert(pos, fuel_list_for_pos);
    }
    for to in 0..max_pos + 1 {
        let mut sum = 0;
        for from in 0..max_pos + 1 {
            sum += pos_to_fuel.get(&from).unwrap()[to as usize];
       }
        if sum > max_cost {
            max_cost = sum;
        }
        if min_cost >  sum {
           min_cost = sum; 
        }
    }


    println!("max cost {}", max_cost);
    println!("min cost {}", min_cost);
}


//i = cost to move all fish from pos to i
fn get_fuel_list_for_pos(pos: i32, max_pos: i32, count: i32) -> Vec<i32> {
   let mut ret: Vec<i32> = vec![0; (max_pos + 1) as usize]; 
   for i in 0..max_pos + 1 {
        let value = if count > 0 { (pos - i).abs() * count } else { 0  };
        ret[i as usize] = value;
   }
   return ret;
}



