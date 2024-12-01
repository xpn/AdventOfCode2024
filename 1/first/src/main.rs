use std::fs;
use std::io::Result;

fn read_challenge_from_file(file_path: &str) -> Result<(Vec<i32>,Vec<i32>)> {

    let mut first_part: Vec<i32> = Vec::new();
    let mut second_part: Vec<i32> = Vec::new();

    let content = fs::read_to_string(file_path)?;
    let lines: Vec<String> = content.lines().map(String::from).collect();
    
    for s in lines {
        let mut parts = s.split_whitespace();
        if let Some(first) = parts.next() {
            first_part.push(first.parse().unwrap());
        }
        if let Some(second) = parts.next() {
            second_part.push(second.parse().unwrap());
        }
    }

    Ok((first_part, second_part))
}

fn main() {
    let mut difference_score = 0;
    let mut similarity_score: i32 = 0;
    let mut list1: Vec<i32>;
    let mut list2: Vec<i32>;
    
    match read_challenge_from_file("input.txt") {
        Ok(results) => {

            list1 = results.0;
            list2 = results.1;

            // Create sorted lists
            list1.sort();
            list2.sort();

            // Sum the differences
            for i in 0..list1.len() {
                difference_score += (list1[i] - list2[i]).abs();
            }

            // Calculate the similarity score
            for i in 0..list1.len() {
                let mut occurances = 0;
                for j in 0..list2.len() {
                    if list1[i] == list2[j] {
                        occurances += 1;
                    }
                }
                similarity_score += (list1[i] * occurances);
            }

            print!("Difference Score: {}\n", difference_score);
            print!("Similarity Score: {}\n", similarity_score);

        }
        Err(e) => eprintln!("Error reading file: {}", e),
    }
}
