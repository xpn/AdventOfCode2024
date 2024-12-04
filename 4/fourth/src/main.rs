use std::fs::*;
use std::collections::HashSet;

fn read_input(path: &str) -> Vec<u8> {
    let result = read(path);

    match (result) {
        Ok(data) => { return data },
        Err(error) => { panic!() }
    }
}

fn window_input_4x4(input: &mut Vec<u8>) -> Vec<[[(u8,usize); 4]; 4]> {

    // We need the length of the width first
    let width = input.iter().position(|&x| x == b'\n').unwrap_or(0);
    let height = input.len() / width;
    let mut result = Vec::new();

    let bytes_to_append = vec![0u8; 1000];
    input.extend(&bytes_to_append);
    for j in 0..height {
        for i in 0..width {
            let array: [[(u8,usize); 4]; 4] = [
                [ (input[(j*height)+i],(j*height)+i), (input[(j*height)+i+1],(j*height)+i+1), (input[(j*height)+i+2],(j*height)+i+2), (input[(j*height)+i+3],(j*height)+i+3) ],
                [ (input[((j+1)*height)+i],((j+1)*height)+i), (input[((j+1)*height)+i+1],((j+1)*height)+i+1), (input[((j+1)*height)+i+2],((j+1)*height)+i+2), (input[((j+1)*height)+i+3],((j+1)*height)+i+3)],
                [ (input[((j+2)*height)+i],((j+2)*height)+i), (input[((j+2)*height)+i+1],((j+2)*height)+i+1), (input[((j+2)*height)+i+2],((j+2)*height)+i+2), (input[((j+2)*height)+i+3],((j+2)*height)+i+3)],
                [ (input[((j+3)*height)+i],((j+3)*height)+i), (input[((j+3)*height)+i+1],((j+3)*height)+i+1), (input[((j+3)*height)+i+2],((j+3)*height)+i+2), (input[((j+3)*height)+i+3],((j+3)*height)+i+3)],
            ];
            result.push(array);
        }
    }

    return result;
} 

fn window_input_3x3(input: &mut Vec<u8>) -> Vec<[[(u8,usize); 3]; 3]> {

    // We need the length of the width first
    let mut width = input.iter().position(|&x| x == b'\n').unwrap_or(0);
    let mut height = (input.len() / width);

    // Remove any newlines from the input now we have the width
    input.retain(|&x| x != b'\n');

    // Add padding as otherwise the X search will wrap around and create false-positives
    for i in 0..height {
        input.insert(width * (height - i), 0);
        input.insert(width * (height - i), 0);
        input.insert(width * (height - i), 0);
        input.insert(width * (height - i), 0);
    }

    // Add on the padding
    width += 4; 
    let mut result = Vec::new();

    // Pad the vector to avoid out of bounds
    let bytes_to_append = vec![0u8; 1000];
    input.extend(&bytes_to_append);

    for j in 0..height {
        for i in 0..width {
            let array: [[(u8,usize); 3]; 3] = [
                [ (input[(j*width)+i],(j*width)+i), (input[(j*width)+i+1],(j*width)+i+1), (input[(j*width)+i+2],(j*width)+i+2) ],
                [ (input[((j+1)*width)+i],((j+1)*width)+i), (input[((j+1)*width)+i+1],((j+1)*width)+i+1), (input[((j+1)*width)+i+2],((j+1)*width)+i+2)],
                [ (input[((j+2)*width)+i],((j+2)*width)+i), (input[((j+2)*width)+i+1],((j+2)*width)+i+1), (input[((j+2)*width)+i+2],((j+2)*width)+i+2)],
            ];
            result.push(array);
        }
    }

    return result;
} 


fn find_words_4x4(input: Vec<[[(u8,usize); 4]; 4]>) -> u32 {

    let mut count = 0;
    let mut coords = Vec::new();

    for matrix in input {
        // Horizontal
        if (matrix[0][0].0 == b'X' && matrix[0][1].0 == b'M' && matrix[0][2].0 == b'A' && matrix[0][3].0 == b'S') ||
           (matrix[0][0].0 == b'S' && matrix[0][1].0 == b'A' && matrix[0][2].0 == b'M' && matrix[0][3].0 == b'X') {
                coords.push(format!("{},{},{},{}",matrix[0][0].1, matrix[0][1].1, matrix[0][2].1, matrix[0][3].1));
        }
        if (matrix[1][0].0 == b'X' && matrix[1][1].0 == b'M' && matrix[1][2].0 == b'A' && matrix[1][3].0 == b'S') ||
           (matrix[1][0].0 == b'S' && matrix[1][1].0 == b'A' && matrix[1][2].0 == b'M' && matrix[1][3].0 == b'X') {
            coords.push(format!("{},{},{},{}",matrix[1][0].1, matrix[1][1].1, matrix[1][2].1, matrix[1][3].1));
        }
        if (matrix[2][0].0 == b'X' && matrix[2][1].0 == b'M' && matrix[2][2].0 == b'A' && matrix[2][3].0 == b'S') ||
           (matrix[2][0].0 == b'S' && matrix[2][1].0 == b'A' && matrix[2][2].0 == b'M' && matrix[2][3].0 == b'X') {
            coords.push(format!("{},{},{},{}",matrix[2][0].1, matrix[2][1].1, matrix[2][2].1, matrix[2][3].1));
        }
        if (matrix[3][0].0 == b'X' && matrix[3][1].0 == b'M' && matrix[3][2].0 == b'A' && matrix[3][3].0 == b'S') ||
           (matrix[3][0].0 == b'S' && matrix[3][1].0 == b'A' && matrix[3][2].0 == b'M' && matrix[3][3].0 == b'X') {
            coords.push(format!("{},{},{},{}",matrix[3][0].1, matrix[3][1].1, matrix[3][2].1, matrix[3][3].1));
        }
        
        // Vertical
        if (matrix[0][0].0 == b'X' && matrix[1][0].0 == b'M' && matrix[2][0].0 == b'A' && matrix[3][0].0 == b'S') ||
           (matrix[0][0].0 == b'S' && matrix[1][0].0 == b'A' && matrix[2][0].0 == b'M' && matrix[3][0].0 == b'X') {
            coords.push(format!("{},{},{},{}",matrix[0][0].1, matrix[1][0].1, matrix[2][0].1, matrix[3][0].1));
            count += 1;
        }
        if (matrix[0][1].0 == b'X' && matrix[1][1].0 == b'M' && matrix[2][1].0 == b'A' && matrix[3][1].0 == b'S') ||
           (matrix[0][1].0 == b'S' && matrix[1][1].0 == b'A' && matrix[2][1].0 == b'M' && matrix[3][1].0 == b'X') {
            coords.push(format!("{},{},{},{}",matrix[0][1].1, matrix[1][1].1, matrix[2][1].1, matrix[3][1].1));
        }
        if (matrix[0][2].0 == b'X' && matrix[1][2].0 == b'M' && matrix[2][2].0 == b'A' && matrix[3][2].0 == b'S') ||
        (matrix[0][2].0 == b'S' && matrix[1][2].0 == b'A' && matrix[2][2].0 == b'M' && matrix[3][2].0 == b'X'){
            coords.push(format!("{},{},{},{}",matrix[0][2].1, matrix[1][2].1, matrix[2][2].1, matrix[3][2].1));
        }
        if (matrix[0][3].0 == b'X' && matrix[1][3].0 == b'M' && matrix[2][3].0 == b'A' && matrix[3][3].0 == b'S') ||
           (matrix[0][3].0 == b'S' && matrix[1][3].0 == b'A' && matrix[2][3].0 == b'M' && matrix[3][3].0 == b'X') {
            coords.push(format!("{},{},{},{}",matrix[0][3].1, matrix[1][3].1, matrix[2][3].1, matrix[3][3].1));
        }

        // Diagnal
        if (matrix[0][0].0 == b'X' && matrix[1][1].0 == b'M' && matrix[2][2].0 == b'A' && matrix[3][3].0 == b'S') ||
           (matrix[0][0].0 == b'S' && matrix[1][1].0 == b'A' && matrix[2][2].0 == b'M' && matrix[3][3].0 == b'X') {
            coords.push(format!("{},{},{},{}",matrix[0][0].1, matrix[1][1].1, matrix[2][2].1, matrix[3][3].1));
        }

        if (matrix[0][3].0 == b'X' && matrix[1][2].0 == b'M' && matrix[2][1].0 == b'A' && matrix[3][0].0 == b'S') ||
           (matrix[0][3].0 == b'S' && matrix[1][2].0 == b'A' && matrix[2][1].0 == b'M' && matrix[3][0].0 == b'X') {
            coords.push(format!("{},{},{},{}",matrix[0][3].1, matrix[1][2].1, matrix[2][1].1, matrix[3][0].1));
        }
    }

    let unique_coords: Vec<_> = coords.into_iter().collect::<HashSet<_>>().into_iter().collect();

    return unique_coords.len().try_into().unwrap();
}

fn find_words_3x3(input: Vec<[[(u8,usize); 3]; 3]>) -> u32 {

    let mut coords = Vec::new();

    for matrix in input {
        // Diagnal
        if (matrix[0][0].0 == b'M' && matrix[1][1].0 == b'A' && matrix[2][2].0 == b'S') &&
           (matrix[2][0].0 == b'S' && matrix[1][1].0 == b'A' && matrix[0][2].0 == b'M') {

            coords.push(format!("{},{},{},{},{},{}",matrix[0][0].1, matrix[1][1].1, matrix[2][2].1, matrix[2][0].1,matrix[1][1].1,matrix[0][2].1));
        }

        if (matrix[0][0].0 == b'S' && matrix[1][1].0 == b'A' && matrix[2][2].0 == b'M') &&
           (matrix[2][0].0 == b'M' && matrix[1][1].0 == b'A' && matrix[0][2].0 == b'S') {
            coords.push(format!("{},{},{},{},{},{}",matrix[0][0].1, matrix[1][1].1, matrix[2][2].1, matrix[2][0].1,matrix[1][1].1,matrix[0][2].1));
        }

        if (matrix[0][0].0 == b'S' && matrix[1][1].0 == b'A' && matrix[2][2].0 == b'M') &&
           (matrix[2][0].0 == b'S' && matrix[1][1].0 == b'A' && matrix[0][2].0 == b'M') {
            coords.push(format!("{},{},{},{},{},{}",matrix[0][0].1, matrix[1][1].1, matrix[2][2].1, matrix[2][0].1,matrix[1][1].1,matrix[0][2].1));
        }

        if (matrix[0][0].0 == b'M' && matrix[1][1].0 == b'A' && matrix[2][2].0 == b'S') &&
           (matrix[2][0].0 == b'M' && matrix[1][1].0 == b'A' && matrix[0][2].0 == b'S') {
            coords.push(format!("{},{},{},{},{},{}",matrix[0][0].1, matrix[1][1].1, matrix[2][2].1, matrix[2][0].1,matrix[1][1].1,matrix[0][2].1));
        }
    }

    // Remove duplicates
    let unique_coords: Vec<_> = coords.into_iter().collect::<HashSet<_>>().into_iter().collect();

    return unique_coords.len().try_into().unwrap();
}

fn print_matrix(inputs: Vec<[[(u8,usize); 3]; 3]>) {
    for input in inputs {
        print!("{}{}{}\n", char::from(input[0][0].0), char::from(input[0][1].0), char::from(input[0][2].0));
        print!("{}{}{}\n", char::from(input[1][0].0), char::from(input[1][1].0), char::from(input[1][2].0));
        print!("{}{}{}\n", char::from(input[2][0].0), char::from(input[2][1].0), char::from(input[2][2].0));
        print!("---------------\n");
    }
}


fn main() {

    let challenge_2 = true;

    let mut input = read_input("/tmp/input.txt");

    if challenge_2 {
        // Remove the stray newline first :/
        input.pop();
        let output = window_input_3x3(&mut input);
        let count = find_words_3x3(output);
        println!("RESULT: {}", count);
    } else {
        let output = window_input_4x4(&mut input);
        let count = find_words_4x4(output);
        println!("RESULT: {}", count);
    }
}
