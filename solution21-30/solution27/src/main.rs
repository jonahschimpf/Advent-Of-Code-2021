use std::fs;
use std::collections::HashMap;
use std::collections::HashSet;
use std::collections::VecDeque;
use std::ascii::AsciiExt;
#[derive(PartialEq, Hash, Eq, Clone, Debug, Copy)]
struct Pair {
    first : char,
    sec: char,
}

impl Pair {
    fn new(first: char, sec: char) -> Pair {
       Pair {first, sec} 
    }
}

fn main() {
    let filename = "input.txt";
    println!("In file {}", filename);
    let contents = fs::read_to_string(filename).unwrap();
    let mut o = contents.split("\n\n").into_iter();

    let start : String = o.next().unwrap().to_string();
    let mut instructions: HashMap<Pair, char> = HashMap::new();
    o.next().unwrap().to_string().lines()
        .for_each(|l| {
            let mut split = l.split(" -> ");
            let chars= split.next().unwrap();
            let x = instructions.insert(Pair::new(chars.chars().nth(0).unwrap(), chars.chars().nth(1).unwrap()), split.next().unwrap().chars().nth(0).unwrap());
            });

    let mut pairs : HashMap<Pair, u64> = HashMap::new();
    for i in 1..start.len() {
       let prev = start.chars().nth(i - 1).unwrap();
       let curr  = start.chars().nth(i).unwrap();
        *pairs.entry(Pair::new(prev, curr)).or_insert(0) += 1;
    }
    for _i in 0..40 {
        pairs = step(&pairs, &instructions);
    }

    let mut max_char = 'A';
    let mut max_count: u64 = 0;
    let mut min_count: u64 = u64::MAX;
    let mut min_char = 'B';

    let mut freq: HashMap<char, u64> = HashMap::new();

    for (k,v) in pairs {
        *freq.entry(k.first).or_insert(0) += v;
        *freq.entry(k.sec).or_insert(0) += v;
    }
    println!("freq is {:?}", freq);
    let copy_freq = freq.clone();
    for (k,v) in copy_freq {
        if k == 'N' || k == 'B' {
            freq.insert(k, ((v - 1) / 2) + 1);
        } else {
            freq.insert(k, v / 2);
        }
    }
    for (k,v) in freq {
        if v > max_count {
            max_count = v;
            max_char = k;
        }
        if v < min_count {
            min_count = v;
            min_char = k;
        }
    }
    println!("Max char is {} min char is {}", max_char, min_char);
    println!("Max char minus min char {}", max_count - min_count);
}

fn step(pairs: &HashMap<Pair, u64>, instructions: &HashMap<Pair, char>) -> HashMap<Pair, u64> {
    let mut to_add : HashMap<Pair, u64> = HashMap::new();
    let mut to_remove : HashSet<Pair> = HashSet::new();
    for (instr_pair, repl) in instructions {
       if pairs.contains_key(&instr_pair) {
           //XY -> Z , pairs(XY, count), pairs(XZ, count)
           *to_add.entry(Pair::new(instr_pair.first, *repl)).or_insert(0) += *pairs.get(instr_pair).unwrap();
           *to_add.entry(Pair::new(*repl, instr_pair.sec)).or_insert(0) += *pairs.get(instr_pair).unwrap();
           to_remove.insert(*instr_pair);
       }
    }
    let mut ret : HashMap<Pair, u64> = pairs.clone();
    for p in to_remove {
        ret.remove(&p);
    }
    for (k ,v) in to_add {
       *ret.entry(k).or_insert(0) += v; 
    }
    return ret;
}
