use std::fs;

fn main() {
    let filename = "input.txt";
    println!("in file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("something went wrong reading the file");

    let binary_rep = convert_to_binary_from_hex(contents);
    println!("Initial binary_rep {}", binary_rep);
    let root = Tree::new(binary_rep, &mut 0);
    root.print();

    println!("here {}", root.sum());
}

struct Tree {
    children: Option<Vec<Box<Tree>>>,
    type_id: u8,
    version: u8,
    bitstring: String,
    value: Option<u64>, //if literal != None else is None
    length_id: Option<char>
}

impl Tree {
    //only extracts the prefix of the string that corresponds to a tree, returns that tree
    fn new(bitstring: String, processed: &mut usize) -> Tree {
        println!("Proccesed is smiley {}", *processed);
        let version: u8 = isize::from_str_radix(&bitstring[*processed + 0..*processed + 3], 2).unwrap() as u8;
        let type_id: u8 = isize::from_str_radix(&bitstring[*processed + 3..*processed + 6], 2).unwrap() as u8;
        println!("Type id is {}", type_id);
        if type_id == 4 { //literal 

        println!("literal {:?}", &bitstring.to_string()[*processed..]);
        *processed += 6;
           let mut i = *processed as usize;
           let mut build: String = "".to_string();
           loop {
                let last_chunk = bitstring.chars().nth(i) == Some('0');
                i += 1;
                for chunk_index in 0..4 {
                    build.push(bitstring.chars().nth(i + chunk_index).unwrap());
                }
                println!("Chunked {}" ,build);
                i += 4;

                *processed += 5;
                if last_chunk {
                    break;
                }
           }

        println!("Proccesed after building {}", *processed);
            println!("build {}", build);
            return Tree {children: None, type_id, version, bitstring, value: Some(get_u64_from_bin_str(build.as_str())), length_id: None};
        } else {
            *processed += 7;
            let remaining_ops: Vec<Box<Tree>> = if bitstring.chars().nth(*processed - 1) == Some('0') {
                Tree::get_ops_0(&bitstring, processed)
            }   else {
                Tree::get_ops_1(&bitstring, processed)
            };
            let copy = bitstring.clone();
            return Tree {children: Some(remaining_ops), type_id, version, bitstring: copy, value: None, length_id: Some(bitstring.chars().nth(6).unwrap())};
        }
    }

    fn get_ops_0(bitstring: &str, processed: &mut usize) -> Vec<Box<Tree>> { 
        println!("ops0 {}", &bitstring[*processed - 7..]);
        let length = get_u64_from_bin_str(&bitstring[*processed..*processed + 15]);
        println!("Length is {}", length);
        let mut trees = Vec::new();
        *processed += 15;
        println!("Processed is {}", processed);
        let old_processed = processed.clone();
        while ((*processed - old_processed) as u64 ) < length {
            trees.push(Box::new(Tree::new(bitstring.to_string(), processed)));
            println!("after pushing processed is now {}", *processed);
        }
        return trees;
    }

    fn get_ops_1(bitstring: &str, processed: &mut usize) -> Vec<Box<Tree>> {
        println!("ops1 {}", &bitstring[*processed - 7..]);
        let chunks = get_u64_from_bin_str(&bitstring[*processed..*processed + 11]);
        let mut trees = Vec::new();
        *processed += 11;
        
        println!("Processed is ops {}", processed);
        println!("Chunks is {}", chunks);
        while trees.len() < chunks as usize {
            trees.push(Box::new(Tree::new(bitstring.to_string(), processed)));
        }
        return trees;
    }

    fn print(&self) {
       if let None = self.children {
            println!("Child: value {}", self.value.unwrap());
       } else {
            println!("Parent");
            for child in self.children.as_ref().unwrap() {
                child.print();
            }
       }
    }

    fn sum(&self) -> u64{
       if let None = self.children {
            self.version as u64
       } else {
            let mut sum = self.version as u64;
            for child in self.children.as_ref().unwrap() {
                sum += child.sum();
            }
            sum
       }
    }

}

fn get_u64_from_bin_str(bin: &str) -> u64 {
    return isize::from_str_radix(bin, 2).unwrap() as u64;
}


fn convert_to_binary_from_hex(hex: String) -> String {
    hex.chars().map(to_binary).collect()
}

fn to_binary(c: char) -> &'static str {
    match c {
        '0' => "0000",
        '1' => "0001",
        '2' => "0010",
        '3' => "0011",
        '4' => "0100",
        '5' => "0101",
        '6' => "0110",
        '7' => "0111",
        '8' => "1000",
        '9' => "1001",
        'A' => "1010",
        'B' => "1011",
        'C' => "1100",
        'D' => "1101",
        'E' => "1110",
        'F' => "1111",
        _ => "",
    }
}



//create a function that takes in a bitstring and creates the Tree from it. 
// pseud: fn (bitstring) -> Tree :
//      type_id, version, listOfOperatoresAsStrings = extract(bitstring) 
//      for operator in listOfOperatorsAsString:
//          complete_tree = fn(operator)
//          ret.children.append(complete_tree)
//      return ret
//
//
//
//      Tree = {ops1, ops0, literals)
