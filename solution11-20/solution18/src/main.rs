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
    let mut discovered: Vec<Vec<bool>> = Vec::new();
    let mut index = 0;
    contents.lines().for_each(|x| {
        matrix.push(Vec::new());
        discovered.push(Vec::new());
        x.chars().into_iter().for_each(|c| {
            matrix[index].push(c.to_digit(RADIX).unwrap() as i32);
            discovered[index].push(false);
        });
    index += 1;
    });
    let mut basin_sum = 0;
    let mut basin_size: Vec<i32> = Vec::new();
    for (i, v) in matrix.iter().enumerate() {
        for (j, d) in v.iter().enumerate() {
            if is_low(i, j, &matrix) {
                basin_size.push(get_size_of_basin(i as i32, j as i32, &matrix, &mut discovered));
            }
        }
    }
    basin_size.sort_by(|a, b| b.cmp(a));
    println!("Count {}", basin_size[0] * basin_size[1] * basin_size[2]);
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

fn get_size_of_basin(i: i32, j:i32, matrix: &Vec<Vec<i32>>, discovered: &mut Vec<Vec<bool>>) -> i32 {
   let row_len = matrix.len() as i32;
   let col_len = matrix[0].len() as i32;
   if j >= col_len || j  < 0 || i < 0 || i >= row_len || discovered[i as usize][j as usize] || (matrix[i as usize][j as usize] == 9) {
        return 0;
   } else {
       let up = (i as i32 - 1) as i32;
       let down = i + 1;
       let left = (j as i32- 1) as i32;
       let right = j + 1;
       discovered[i as usize][j as usize] = true;
       return 1 + get_size_of_basin(up, j, matrix, discovered) + get_size_of_basin(down, j, matrix, discovered) +
           get_size_of_basin(i, left, matrix, discovered) + get_size_of_basin(i, right, matrix, discovered);
   }
}
