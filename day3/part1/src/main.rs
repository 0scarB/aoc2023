use std::{env, fs::File, io::{self, BufRead}};

const ADJECENT_CELLS_IDX_OFFSETS: [(i8, i8); 8] = [
    (-1, -1),
    (-1,  0),
    (-1,  1),
    ( 0, -1),
    ( 0,  1),
    ( 1, -1),
    ( 1,  0),
    ( 1,  1),
];

type CharsMatrix = Vec<Vec<char>>;

struct CharWithInfo<'a> {
    char_: &'a char,
    is_digit: bool,
    is_adjacent_to_symbol: bool,
}

type CharsWithInfoMatrix<'a> = Vec<Vec<CharWithInfo<'a>>>;

#[derive(Debug, PartialEq)]
struct SchematicNum {
    num: usize,
    is_adjacent_to_symbol: bool,
}

fn main() -> Result<(), String> {
    let file_path = env::args().nth(1).ok_or("No file provided!")?;
    let file = File::open(file_path).map_err(|err| err.to_string())?;
    let file_lines = io::BufReader::new(file).lines()
        .map(|l| l.unwrap());
    let sum: usize = lines_to_schematic_numbers(file_lines).iter()
        .filter(|num| num.is_adjacent_to_symbol)
        .map(|num| num.num)
        .sum();
    println!("Sum of part numbers: {}", sum);

    Ok(())
}

fn lines_to_schematic_numbers<'a>(
    lines: impl Iterator<Item = String>,
) -> Vec<SchematicNum> {
    let chars_mat = lines_to_chars_matrix(lines);
    let chars_info_mat = add_info_to_chars_matrix(&chars_mat);
    chars_with_info_matrix_to_schematic_numbers(chars_info_mat)
}

fn add_info_to_chars_matrix<'a>(mat: &'a CharsMatrix) -> CharsWithInfoMatrix<'a> {
    let mut info_mat = Vec::new();
    for (i, row) in mat.iter().enumerate() {
        let mut info_row = Vec::new();
        for (j, c) in row.iter().enumerate() {
            info_row.push(CharWithInfo {
                char_: c,
                is_digit: '0' <= *c && *c <= '9',
                is_adjacent_to_symbol:
                    ADJECENT_CELLS_IDX_OFFSETS.iter().any(|(i_offset, j_offset)| {
                        mat.get(((i as i64) + (*i_offset as i64)) as usize).map(|r| {
                            r.get(((j as i64) + (*j_offset as i64)) as usize)
                                .map(|c| {
                                    (*c < '0' || '9' < *c) && *c != '.'
                                }).unwrap_or(false)
                        }).unwrap_or(false)
                    }),
            });
        }
        info_mat.push(info_row);
    }
    info_mat
}

fn chars_with_info_matrix_to_schematic_numbers(
    mat: CharsWithInfoMatrix,
) -> Vec<SchematicNum> {
    let mut part_numbers = Vec::<SchematicNum>::new();
    let mut num_digits = Vec::<&char>::new();
    let mut is_in_num = false;
    let mut is_adjacent_to_symbol = false;
    for row in mat {
        for cell in row {
            if is_in_num {
                if cell.is_digit {
                    num_digits.push(cell.char_);
                    is_adjacent_to_symbol = is_adjacent_to_symbol || cell.is_adjacent_to_symbol;
                } else {
                    let mut num = 0;
                    for dig in &num_digits {
                        num = num*10 + (**dig as usize - '0' as usize);
                    }
                    part_numbers.push(SchematicNum {
                        num,
                        is_adjacent_to_symbol,
                    });
                    num_digits.clear();
                    is_in_num = false;
                    is_adjacent_to_symbol = false;
                }
            } else {
                if cell.is_digit {
                    num_digits.push(cell.char_);
                    is_in_num = true;
                    is_adjacent_to_symbol = cell.is_adjacent_to_symbol;
                }
            }
        }
    }
    if is_in_num {
        let mut num = 0;
        for dig in num_digits {
            num = num*10 + (*dig as usize - '0' as usize);
        }
        part_numbers.push(SchematicNum {
            num,
            is_adjacent_to_symbol,
        });
    }
    part_numbers
}

fn lines_to_chars_matrix<'a>(lines: impl Iterator<Item = String>) -> CharsMatrix {
    let mut mat = Vec::new();
    for line in lines {
        mat.push(line.chars().collect());
    }
    return mat
}

#[cfg(test)]
mod tests {
    use crate::{lines_to_schematic_numbers, SchematicNum};

    #[test]
    fn test_lines_to_schematic_numbers() {
        assert_eq!(
            lines_to_schematic_numbers([
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
            ].iter().map(|l| l.to_string())),
            vec![
                SchematicNum {num: 467, is_adjacent_to_symbol: true},
                SchematicNum {num: 114, is_adjacent_to_symbol: false},
                SchematicNum {num: 35, is_adjacent_to_symbol: true},
                SchematicNum {num: 633, is_adjacent_to_symbol: true},
                SchematicNum {num: 617, is_adjacent_to_symbol: true},
                SchematicNum {num: 58, is_adjacent_to_symbol: false},
                SchematicNum {num: 592, is_adjacent_to_symbol: true},
                SchematicNum {num: 755, is_adjacent_to_symbol: true},
                SchematicNum {num: 664, is_adjacent_to_symbol: true},
                SchematicNum {num: 598, is_adjacent_to_symbol: true},
            ],
        );
    }
}
