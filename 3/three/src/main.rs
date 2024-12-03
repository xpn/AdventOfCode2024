use std::fs::read_to_string;
use regex::{Error, Regex};

fn read_input(path: &str) -> String {
    let data = read_to_string(path);

    match data {
        Ok(result) => return result,
        Err(error ) => panic!()
    }
}

fn create_batches(data: &String) -> String {
    let mut strbuilder = String::new();
    let mut toggle = true;

    for i in 0..data.len() {

        let substr = &data[i..];
        if substr.starts_with("do()") {
            toggle = true;
        }
        if substr.starts_with("don't()") {
            toggle = false;
        }

        if toggle {
            if let Some((index, c)) = data.char_indices().nth(i) {
                strbuilder.push(c);
            }
        }
    }

    return strbuilder;
}

fn match_all_muls(data: &String) -> i32 {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let mut total = 0;

    for captures in re.captures_iter(data) {
        let left_str = captures.get(1).map_or("", |m| m.as_str());
        let right_str = captures.get(2).map_or("", |m| m.as_str());

        let left: i32 = left_str.parse().expect("Could not convert");
        let right: i32 = right_str.parse().expect("Could not convert");
        
        total += (left * right);
    }

    return total;

}

fn main() {

    let challenge_2 = false;

    let mut data = read_input("/tmp/input.txt");
    if challenge_2 {
        data = create_batches(&data);
    }
    let result = match_all_muls(&data);

    println!("Result: {}", result);
}
