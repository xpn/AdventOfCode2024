use std::fs::read_to_string;
use std::io::Result;

fn load_input(file_path: &str) -> Result<Vec<Vec<i32>>> {

    let mut return_val : Vec<Vec<i32>> = Vec::new();

    let input_data = read_to_string(file_path)?;

    let lines: Vec<String> = input_data.lines().map(String::from).collect();

    for l in lines {
        let parts = l.split_whitespace();
        let arr : Vec<i32> = parts.map(|s| s.parse().unwrap()).collect::<Vec<i32>>().try_into().unwrap();
        return_val.push(arr);
    }

    Ok(return_val)
}

fn is_increasing(report: &Vec<i32>) -> bool {

    let mut increasing = 0;
    let mut decreasing = 0;

    if report.len() < 2 {
        return false;
    }

    for i in 1..report.len() {
        if report[i-1] > report[i] {
            decreasing += 1;
        } else {
            increasing += 1;
        }
    }

    if decreasing >= increasing {
        return false
    } else {
        return true
    }
}

fn is_safe(report: &Vec<i32>) -> bool {

    let mut report_mut = report.clone();

    if is_increasing(report) {

        let mut first = report_mut.remove(0);

        while report_mut.len() != 0 {
            let second = report_mut.remove(0);

            let diff = second - first;

            if diff <= 0 || diff > 3 {
                return false
            }

            first = second;
        }
        
    } else  {

        let mut first = report_mut.remove(0);

        while report_mut.len() != 0 {
            let second = report_mut.remove(0);

            let diff = first - second;

            if diff <= 0 || diff > 3 {
                return false;
            }
            
            first = second;
        }
    }

    return true;

}

fn main() {
    let mut count = 0;

    let challenge_2 = true;

    match load_input("/tmp/input.txt") {
        Ok(results) => {
            for input in results {
                if is_safe(&input) {
                    count += 1
                } else {
                    if challenge_2 {
                        for i in 0..input.len() {

                            let mut input_copy = input.clone();
                            input_copy.remove(i);

                            if is_safe(&input_copy) {
                                count += 1;
                                break;
                            }
                        }
                    }
                }
            }
        }
        Err(e) => eprintln!("error reading file: {}", e),
    }

    println!("Number Of Safe: {}", count);
}
