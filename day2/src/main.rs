
fn exist(v : &Vec<usize>, searched : usize) -> bool {
    for i in v.iter() {
        if *i == searched {
            return true;
        }
    }
    return false;
}

fn solution_1( data: &str ) -> i32 {
    let mut twice : i32 = 0;
    let mut thrice : i32 = 0;
    for line in data.lines() {
        let mut counters = vec![0; 26];
        for chr in line.chars() {
            let index = (chr as usize) - 97;
            counters[index] += 1;
        }
        if exist(&counters, 2) {
            twice += 1;
        }
        if exist(&counters, 3) {
            thrice += 1;
        }
    }
    twice * thrice
}

fn distance(first : &str, second : &str) -> Vec<(char, char)> {
    first.chars().zip(second.chars()).filter(|(a, b)| { a == b }).collect()
}

fn solution_2(data: &str) -> String {
    let mut storage : Vec<&str> = Vec::new();
    for line in data.lines() {
        storage.push(line);
    }
    for i in storage.clone() {
        for j in storage.clone() {
            if i == j {
                continue;
            } else {
                let dist = distance(i, j);
                if dist.len() == 25 {
                    return dist.into_iter().map(|(a, _)| a ).collect::<String>()    
                } 
            }
        } 
    }

    "".to_owned()
}

fn main() {
    let data = include_str!("../inputs/input2.txt");
    println!("Solution to problem 2.A: {}", solution_1(&data));
    println!("Solution to problem 2.B: {}", solution_2(&data));
}
