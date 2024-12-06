use std::{fs::read, ops::Index};
use std::thread;
use std::time::Duration;
use std::process::Command;
use threadpool::ThreadPool;

enum PlayerDirection {
    Up,
    Down,
    Left,
    Right,
    OffMap,
}

fn read_input(path: &str) -> Vec<u8> {
    return read(path).unwrap();
}

fn build_map(input: &Vec<u8>) -> Vec<Vec<u8>> {
    // Find width
    let output = input.split(|e| *e == b'\n').map(|e| e.to_vec()).collect();

    return output;
}

// Returns x,y position of player
fn get_player_position(input: &Vec<Vec<u8>>) -> (usize, usize, PlayerDirection) {
    // Search through map to find the player icon
    for y in 0..input.len() {
        for x in 0..input[y].len() {
            if input[y][x] == b'>' {
                return (y,x,PlayerDirection::Right);
            } else if input[y][x] == b'<' {
                return (y,x,PlayerDirection::Left);
            } else if input[y][x] == b'^' {
                return (y,x,PlayerDirection::Up);
            } else if input[y][x] == b'v' {
                return (y,x,PlayerDirection::Down);
            } 
        }
    }

    return (0,0,PlayerDirection::OffMap);
}

// Return None if OOB
// Return true if hit an object
// Return false if no hit
fn collision(input: &Vec<Vec<u8>>) -> Option<bool> {
    let position = get_player_position(input);

    let y = position.0;
    let x = position.1;
    let x_max = input[0].len() - 1;
    let y_max = input.len() - 1;

    match(position.2) {
        PlayerDirection::Up => {
            if y == 0 {
                return None;
            }

            if input[y-1][x] == b'#' {
                return Some(true);
            }         
        }
        PlayerDirection::Down => {
            if y + 1 > y_max {
                return None;
            }
            
            if input[y+1][x] == b'#' {
                return Some(true);
            } 
        }
        PlayerDirection::Left => {
            if x == 0 {
                return None;
            }

            if input[y][x-1] == b'#' {
                return Some(true);
            } 
        }
        PlayerDirection::Right => {
            if x + 1 > x_max {
                return None;
            }

            if input[y][x+1] == b'#' {
                return Some(true);
            } 
        }
        PlayerDirection::OffMap => {
            return None;
        }
    }    

    return Some(false);

}

fn move_player(input: &mut Vec<Vec<u8>>, tread_count: &mut usize) -> bool {
    let player_position = get_player_position(input);
    let y = player_position.0;
    let x = player_position.1;
    let x_max = input[0].len() - 1;
    let y_max = input.len() - 1;
    let direction = player_position.2;
    let orig_direction = get_player_position(input).2;

    // Check for collision first
    let collision_hit = collision(input);

    if let Some(d) = collision_hit {
        if d == true {
            match(direction) {
                PlayerDirection::Up => {
                    input[y][x] = b'>';
                }
                PlayerDirection::Down => {
                    input[y][x] = b'<';
                }
                PlayerDirection::Left => {
                    input[y][x] = b'^';
                }
                PlayerDirection::Right => {
                    input[y][x] = b'v';
                }
                PlayerDirection::OffMap => {
                    
                }
            }
            return true;
        }
    } else {
        // Off map
        return false;
    }

    // No collision so we move
    match(direction) {
        PlayerDirection::Up => {
            if y == 0 {
                return false;
            }
            if input[y-1][x] == b'.' {
                *tread_count = 0;
            } else {
                *tread_count += 1;
            }
            input[y-1][x] = b'^';
        }
        PlayerDirection::Down => {
            if y + 1 > y_max {
                return false;
            }
            if input[y+1][x] == b'.' {
                *tread_count = 0;
            } else {
                *tread_count += 1;
            }

            input[y+1][x] = b'v';
        }
        PlayerDirection::Left => {
            if x == 0 {
                return false;
            }
            if input[y][x-1] == b'.' {
                *tread_count = 0;
            } else {
                *tread_count += 1;
            }
            input[y][x-1] = b'<';
        }
        PlayerDirection::Right => {
            if x+1 > x_max {
                return false;
            }
            if input[y][x+1] == b'.' {
                *tread_count = 0;
            } else {
                *tread_count += 1;
            }
            input[y][x+1] = b'>';
        }
        PlayerDirection::OffMap => {
            return false;
        }
    }

    match orig_direction {
        PlayerDirection::Up => {
            input[y][x] = b'X';
        }
        PlayerDirection::Down => {
            input[y][x] = b'X';
        }
        PlayerDirection::Left => {
            input[y][x] = b'X';
        }
        PlayerDirection::Right => {
            input[y][x] = b'X';
        }
        PlayerDirection::OffMap => {
        
        }
    }

    return true;
}

fn print_map(map: &Vec<Vec<u8>>) {
    for y in 0..map.len() {
        for x in 0..map[y].len() {
            print!("{}", char::from(map[y][x]));
        }
        print!("\n");
    }
}

fn get_potential_obsticle_locations(map: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    // We should only place obsticles on previously walked paths, otherwise it's pointless

    let mut paths = Vec::new();

    let x_max = map[0].len();
    let y_max = map.len();

    for y in 0..y_max {
        for x in 0..x_max {
            if map[y][x] == b'X' {
                paths.push((y,x));
            } else if map[y][x] == b'v' || map[y][x] == b'^' || map[y][x] == b'<' || map[y][x] == b'>'{
                paths.push((y,x));
            } else {
                paths.push((y,x));
            }

        }
    }

    return paths;
}

fn clear_terminal() {
    Command::new("clear")
        .status()
        .expect("Failed to clear the terminal");
}

fn plant_obsticle(map: &mut Vec<Vec<u8>>, x: usize, y: usize) {
    map[y][x] = b'#';
}

fn main() {
    //let mut thread_handles = Vec::new();
    let mut in_play = true;
    
    let data = read_input("/tmp/input.txt");
    let mut map = build_map(&data);
    let mut tread_count = 0;
    while in_play {
        in_play = move_player(&mut map, &mut tread_count);
        
    }
    print_map(&map);

    // Get potential obsticle paths
    let possible_obsticle_paths = get_potential_obsticle_locations(&map);
    println!("Steps in Original Path (Answer 1): {}", possible_obsticle_paths.len());

    let pool = ThreadPool::new(50);

    for coords in possible_obsticle_paths {
        pool.execute(move || {
            let data = read_input("/tmp/input.txt");
            let mut map = build_map(&data);
            let player_start = get_player_position(&map);
            if coords.0 == player_start.0 && coords.1 == player_start.1 {
                // We ignore our starting position
                return;
            }
            plant_obsticle(&mut map, coords.1, coords.0);
            let mut in_play = true;
            let mut tread_count = 0;
            while in_play {
                in_play = move_player(&mut map, &mut tread_count);
                //print_map(&map);
                //clear_terminal();
                if tread_count > 10000 {
                    println!("LOOP DETECTED!!");
                    return;
                }
            }

            // let data2 = read_input("/tmp/input.txt");
            // let mut map2 = build_map(&data2);
            // check_map(&map2, &map);     
    });
    }

    pool.join();
}
