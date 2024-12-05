// Not too sure how I did this one.. Man-Flu in full swing so it's a blur xD

use std::{fs::read_to_string, ops::Index, str::FromStr};
use regex::Regex;
use std::collections::HashMap;

fn load_input(input: &str) -> String {
    let data = read_to_string(input);
    match(data) {
        Ok(d) => return d,
        Err(e) => panic!(),
    }
}

fn parse_rules(input: &String) -> HashMap<u32, Vec<u32>> {

    let mut rules: HashMap<u32, Vec<u32>> = HashMap::new();

    // Use regex to capture all the rules
    let re = Regex::new(r"(\d+)\|(\d+)").unwrap();

    for capture in re.captures_iter(input.as_str()) {
        let group1 = capture.get(1).map_or("", |m| m.as_str());
        let group2 = capture.get(2).map_or("", |m| m.as_str());

        let mut group1_int = 0;
        let mut group2_int = 0;

        match group1.parse::<u32>() {
            Ok(value) => group1_int = value,
            Err(err) => panic!(),
        }

        match group2.parse::<u32>() {
            Ok(value) => group2_int = value,
            Err(err) => panic!(),
        }

        // Check if already exists
        if rules.contains_key(&group1_int) {
            // We update the vector
            rules.get_mut(&group1_int).unwrap().push(group2_int);
        } else {
            let mut new_rules = vec!();
            new_rules.push(group2_int);
            rules.insert(group1_int, new_rules);
        }
    }

    return rules;

}

fn return_middle_number(input: String) -> u32 {
    let v2: Vec<&str> = input.split(",").collect();
    let m= v2[v2.len() / 2];
    return m.parse::<u32>().unwrap()
}

fn order_by_rules(input: &mut String, rules: &HashMap<u32, Vec<u32>>) -> String{
    
   let input_split= input.split(",");

   let mut arr_output: Vec<u32> = input_split.filter_map(|s: &str| s.parse::<u32>().ok()).collect();

    for i in 0..arr_output.len() {
        let check_val = arr_output[i];
        if rules.contains_key(&check_val) {
            let rule_vals = rules.get(&check_val).unwrap();
            println!("RULES: {:?}\n", rule_vals);
            for r in rule_vals {
                // Check if the r value occurs after our current value
                if arr_output[..i].contains(r) {
                    let b = arr_output.iter().position(|&x| x == *r).unwrap();
                    arr_output.swap(i, b);
                }
            }
        }
    }

    let result = arr_output.iter()
        .map(|&x| x.to_string())
        .collect::<Vec<String>>()
        .join(",");

    return result;

}

fn validate_order(question: &String, rules: &HashMap<u32, Vec<u32>>) -> bool {

    let mut previous_question_pages = vec!();
    let mut violation = false;
    for question_page in question.split(",") {
        let question_page_int = question_page.parse::<u32>().unwrap();
        if rules.contains_key(&question_page_int) {
            // check the rules against everything that came before
            for r in rules.get(&question_page_int).unwrap() {
                if previous_question_pages.contains(r) {
                    // Rule violation
                    violation = true;
                }
            }
        }

        previous_question_pages.push(question_page_int);
    }

    return violation;
}

fn parse_questions(data: &String, rules: HashMap<u32, Vec<u32>>) {
    
    let mut answer = 0;
    let mut valid = vec!();
    let mut invalid = vec!();

    // First we find the answers
    let index = data.find("\n\n").unwrap_or(0);

    let questions = &data[index+2..];

    for question in questions.lines() {

        let violation = validate_order(&question.to_string(), &rules);

        // Check if this was valid or not
        if !violation {
            valid.push(question);
        } else {
            invalid.push(question);
        }
    }

    // Now grab all the middle numbers and sum
    for v in valid {
        answer += return_middle_number(v.to_string());
    }

    // For part 2 we need to focus on the invalid answers
    // we're going to swap based on the rules
    answer = 0;
    for question in invalid {
        let mut qconv = String::from_str(question).unwrap();

        while true {
            
            let fixed = order_by_rules(&mut qconv, &rules);

            if validate_order(&fixed, &rules) == false {
                qconv = fixed;
                break;
            }

            qconv = fixed;
        }

        answer += return_middle_number(qconv.to_string());
    }

    println!("Answer 2: {}", answer);

}

fn main() {
    let data = load_input("/tmp/input.txt");

    let rules = parse_rules(&data);

    parse_questions(&data, rules);
}
