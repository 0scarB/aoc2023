use std::{env, fs::File, io::{self, BufRead}};

fn main() -> Result<(), String> {
    let match_chars_to_int: Vec<(Vec<char>, usize)> = vec![
        (vec!['1'], 1),
        (vec!['2'], 2),
        (vec!['3'], 3),
        (vec!['4'], 4),
        (vec!['5'], 5),
        (vec!['6'], 6),
        (vec!['7'], 7),
        (vec!['8'], 8),
        (vec!['9'], 9),
        (vec!['o', 'n', 'e'], 1),
        (vec!['t', 'w', 'o'], 2),
        (vec!['t', 'h', 'r', 'e', 'e'], 3),
        (vec!['f', 'o', 'u', 'r'], 4),
        (vec!['f', 'i', 'v', 'e'], 5),
        (vec!['s', 'i', 'x'], 6),
        (vec!['s', 'e', 'v', 'e', 'n'], 7),
        (vec!['e', 'i', 'g', 'h', 't'], 8),
        (vec!['n', 'i', 'n', 'e'], 9),
    ];
    let mut match_chars_rev_to_int: Vec<(Vec<char>, usize)> = Vec::new();
    for i in 0..match_chars_to_int.len() {
        match_chars_rev_to_int.push((
            Vec::from_iter(match_chars_to_int[i].0.iter().map(|c| *c).rev()),
            match_chars_to_int[i].1
        ));
    }
    let file_path = env::args().nth(1).ok_or("No file provided!")?;
    let file = File::open(file_path).map_err(|err| err.to_string())?;
    let file_lines = io::BufReader::new(file).lines()
        .map(|l| l.unwrap());
    let sum = file_lines
        .map(|l| calibration_value(&match_chars_to_int, &match_chars_rev_to_int, l))
        .sum::<Result<usize, String>>()?;
    println!("Calibration value sum: {}", sum);

    Ok(())
}

fn calibration_value(
    match_chars_to_int: &Vec<(Vec<char>, usize)>, 
    match_chars_rev_to_int: &Vec<(Vec<char>, usize)>, 
    line: String,
) -> Result<usize, String> {
    let err_msg = "No digit on line!";
    let tens_place = return_int_on_match(match_chars_to_int, &Vec::from_iter(line.chars())).ok_or(err_msg)?;
    let ones_place = return_int_on_match(match_chars_rev_to_int, &Vec::from_iter(line.chars().rev())).ok_or(err_msg)?;
    Ok(10 * tens_place + ones_place)
}

fn return_int_on_match(match_chars_to_int: &Vec<(Vec<char>, usize)>, chars: &Vec<char>) -> Option<usize> {
    for i in 0..chars.len() {
        'x: for (match_chars, int) in match_chars_to_int {
            for j in 0..match_chars.len() {
                if i + j >= chars.len() || match_chars[j] != chars[i + j] {
                    continue 'x;
                }
            }
            return Some(*int);
        }
    }
    None
}
