use std::fs::read_to_string;
use regex::Regex;
use itertools::{repeat_n, Itertools};

fn load_file(path: &str) -> String {
    let data = read_to_string(path).unwrap();

    return data;
}

fn process_data(data: &String) -> Vec<(usize, Vec<usize>)> {

    let re = Regex::new(r"(\d+): ([^\n]+)").unwrap();
    let mut processed = Vec::new();

    for capture in re.captures_iter(data) {
        let answer = capture.get(1).map_or("", |m| m.as_str());
        let question = capture.get(2).map_or("", |m| m.as_str());
        let questions = question.split(" ");

        let mut q_vector = Vec::new();
        for q in questions {
            let part = q.parse().unwrap();
            q_vector.push(part);
        }
        processed.push((answer.parse().unwrap(), q_vector));

    }

    return processed;
}

type CalcFunc = fn(usize, usize) -> usize;

fn addition(val1: usize, val2: usize) -> usize {
    //println!("{} + {}", val1, val2);
    val1 + val2
}

fn multiply(val1: usize, val2: usize) -> usize {
    //println!("{} * {}", val1, val2);
    val1 * val2
}

fn concatstr(val1: usize, val2: usize) -> usize {
    let result = format!("{}{}", val1, val2);
    return result.parse().unwrap();

}

fn permutate(operators: [CalcFunc;3], numbers: Vec<usize>, solution: usize) -> bool {

    let number_count = numbers.len();

    // Create all possible combinations of operators
    let operator_combinations = repeat_n(operators, number_count-1).multi_cartesian_product();//operators.iter().cloned().combinations_with_replacement(number_count-1);//operators.iter().cloned().cycle().take(number_count - 1).permutations(number_count - 1);

    for c in operator_combinations {
        let mut answer = numbers[0];
        for (i, op) in c.iter().enumerate() {
            answer = op(answer, numbers[i+1]);
        }
        if answer == solution {
            return true;
        }
    }

    return false;
}


fn solve_all(input: Vec<(usize, Vec<usize>)>) {

    let mut answer = 0;
    for sum in input {
        if permutate([addition, multiply, concatstr], sum.1, sum.0) {
            answer += sum.0
        }
    }

    println!("Answer: {}", answer);
}


fn main() {
    let data = load_file("/tmp/input.txt");
    let processed = process_data(&data);
    solve_all(processed);
}
