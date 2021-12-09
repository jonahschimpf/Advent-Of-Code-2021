use std::fs;
use std::collections::HashSet;

use std::collections::HashMap;
fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);
    const RADIX: u32 = 10;
    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");
    let mut matrix: Vec<Vec<i32>> = Vec::new();

    let mut index = 0;
    contents.lines().for_each(|x| {
        matrix.push(Vec::new());
        x.chars().into_iter().for_each(|c| {
            matrix[index].push(c.to_digit(RADIX).unwrap() as i32);
        });
    index += 1;
    });
    let mut low_sum = 0;
    for (i, v) in matrix.iter().enumerate() {
        for (j, d) in v.iter().enumerate() {
            if is_low(i, j, &matrix) {
                low_sum += (d + 1);
            }
        }
    }
    println!("Count {}", low_sum);
}

fn is_low(i: usize, j: usize, matrix: &Vec<Vec<i32>>) -> bool {
   let row_len = matrix.len();
   let col_len = matrix[0].len();
   let value = matrix[i][j];
   println!("{}", j);
   let up = (i as i32 - 1) as i32;
   let down = i + 1;
   let left = (j as i32- 1) as i32;
   let right = j + 1;
   let mut up_low = false;
   let mut down_low = false;
   let mut right_low = false;
   let mut left_low = false;
   if up >= 0 {
       up_low = value < matrix[up as usize][j]; 
    } else {
        up_low = true;
   }
   if down < row_len {
       down_low = value < matrix[down][j]; 
   } else {
        down_low = true;
   }
   if left >= 0 {
       left_low = value < matrix[i][left as usize]; 
   } else {
        left_low = true;
   }
   if right < col_len {
       right_low = value < matrix[i][right]; 
   } else {
        right_low = true;
   }
   return up_low & down_low & left_low & right_low;
}
