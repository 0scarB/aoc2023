use std::{fs::File, io::Read};

#[derive(Debug, PartialEq)]
enum Token {
    Num(usize),
    Gear,
    Unknown,
}

#[derive(Debug, PartialEq)]
struct TokenMatrix {
    idx_matrix: Vec<Vec<usize>>,
    tokens: Vec<Token>,
}

fn main() -> Result<(), String> {
    let mut file = File::open("../input.txt").map_err(|err| err.to_string())?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    println!("Sum of gear ratios: {}", sum_gear_ratios(&input));

    Ok(())
}

fn sum_gear_ratios(input: &str) -> usize {
    let token_matrix = tokenize(input);
    let mut sum = 0;
    for row_idx in 0..token_matrix.idx_matrix.len() {
        for col_idx in 0..token_matrix.idx_matrix[row_idx].len() {
            gear_ratio_at_cell(&token_matrix, row_idx, col_idx).map(|x| sum += x);
        }
    }
    sum
}

fn tokenize(input: &str) -> TokenMatrix {
    let mut idx_matrix = Vec::<Vec<usize>>::new();
    let mut idx_matrix_row = Vec::<usize>::new();
    let mut tokens = Vec::<Token>::new();
    let mut token_idx = 0;

    let mut is_in_num = false;
    let mut num_chars = Vec::<char>::new();
    let mut num_len = 0;

    for char in input.chars() {
        if is_in_num {
            if '0' <= char && char <= '9' {
                num_chars.push(char);
                num_len += 1;
                continue;
            }

            let mut num = 0;
            for num_char in &num_chars {
                num = 10*num + (*num_char as usize - '0' as usize);
            }
            let token = Token::Num(num);
            for _ in 0..num_len {
                idx_matrix_row.push(token_idx);
            }
            tokens.push(token);
            token_idx += 1;
            is_in_num = false;
            num_chars.clear();
            num_len = 0;
        }

        if '0' <= char && char <= '9' {
            is_in_num = true;
            num_chars.push(char);
            num_len = 1;
            continue;
        }

        if char == '\n' {
            idx_matrix.push(idx_matrix_row.clone());
            idx_matrix_row.clear();
            continue;
        }

        if char == '*' {
            let token = Token::Gear;
            idx_matrix_row.push(token_idx);
            tokens.push(token);
            token_idx += 1;
            continue;
        }

        let token = Token::Unknown;
        idx_matrix_row.push(token_idx);
        tokens.push(token);
        token_idx += 1;
    }

    if is_in_num {
        let mut num = 0;
        for num_char in &num_chars {
            num = 10*num + (*num_char as usize - '0' as usize);
        }
        let token = Token::Num(num);
        for _ in 0..num_len {
            idx_matrix_row.push(token_idx);
        }
        tokens.push(token);
    }

    if idx_matrix.len() > 0 {
        idx_matrix.push(idx_matrix_row.clone());
        idx_matrix_row.clear();
    }
    
    TokenMatrix {idx_matrix, tokens}
}

fn gear_ratio_at_cell(token_matrix: &TokenMatrix, row: usize, col: usize) -> Option<usize> {
    let token_idx = token_matrix.idx_matrix[row][col];
    let _ = match token_matrix.tokens[token_idx] {
        Token::Gear => Some(()),
        _ => None
    }?;
    let mut adj_num_count = 0;
    let mut gear_ratio = 1;
    let mut encountered_adj_num_idxs = Vec::<usize>::new();
    for (adj_row, adj_col) in [
        (row - 1, col - 1),
        (row - 1, col    ),
        (row - 1, col + 1),
        (row    , col - 1),
        (row    , col + 1),
        (row + 1, col - 1),
        (row + 1, col    ),
        (row + 1, col + 1),
    ] {
        let opt_adj_token_idx = token_matrix.idx_matrix.get(adj_row).map(|r| r.get(adj_col)).flatten();
        if let Some(adj_token_idx) = opt_adj_token_idx {
            if let Token::Num(adj_num) = token_matrix.tokens[*adj_token_idx] {
                if encountered_adj_num_idxs.contains(adj_token_idx) {
                    continue;
                }
                gear_ratio *= adj_num;
                adj_num_count += 1;
                encountered_adj_num_idxs.push(*adj_token_idx);
            }
        }
    }

    if adj_num_count != 2 {
        return None;
    }

    return Some(gear_ratio);
}

#[cfg(test)]
mod tests {
    use crate::{TokenMatrix, Token, tokenize, sum_gear_ratios};

    #[test]
    fn test_lines_to_schematic_numbers() {
        let input = [
            "467..",
            "...*.",
            "..35.",
        ].join("\n");
        assert_eq!(
            tokenize(&input),
            TokenMatrix {
                idx_matrix: vec![
                    vec![0, 0, 0, 1, 2],
                    vec![3, 4, 5, 6, 7],
                    vec![8, 9, 10, 10, 11],
                ],
                tokens: vec![
                    Token::Num(467),
                    Token::Unknown,
                    Token::Unknown,
                    Token::Unknown,
                    Token::Unknown,
                    Token::Unknown,
                    Token::Gear,
                    Token::Unknown,
                    Token::Unknown,
                    Token::Unknown,
                    Token::Num(35),
                    Token::Unknown,
                ],
            },
        );
    }

    #[test]
    fn test_sum_gear_ratios() {
        let input = [
            "467..114..",
            "...*......",
            "..35..633.",
            "......#...",
            "617*......",
            ".....+.58.",
            "..592.....",
            "......755.",
            "...$.*....",
            ".664.598..",
        ].join("\n");
        assert_eq!(
            sum_gear_ratios(&input),
            467835,
        );
    }
}
