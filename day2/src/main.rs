use std::{io::{self, BufRead}, env, fs::File};

#[derive(Debug, PartialEq)]
struct ColorCounts {
    red_count: usize,
    green_count: usize,
    blue_count: usize,
}

type Handful = ColorCounts;

#[derive(Debug, PartialEq)]
struct Game {
    id: usize,
    handfuls: Vec<Handful>,
}

fn main() -> Result<(), String> {
    let max_color_counts = ColorCounts {
        red_count: 12,
        green_count: 13,
        blue_count: 14,
    };

    let file_path = env::args().nth(1).ok_or("No file provided!")?;
    let file = File::open(file_path).map_err(|err| err.to_string())?;
    let file_lines = io::BufReader::new(file).lines()
        .map(|l| l.unwrap());
    let sum_possible_game_ids: usize =
        file_lines.map(|l| parse_line(l.as_str()).unwrap())
        .filter(|game| is_possible_game(&max_color_counts, game))
        .map(|game| game.id)
        .sum();
    println!("Sum of IDs of possible games: {}", sum_possible_game_ids);
    Ok(())
}

fn is_possible_game(
    max_color_counts: &ColorCounts, 
    game: &Game
) -> bool {
    return game.handfuls.iter()
        .all(|handful| is_possible_handful(max_color_counts, handful))
}

fn is_possible_handful(
    max_color_counts: &ColorCounts,
    handful: &Handful,
) -> bool {
    handful.red_count           <= max_color_counts.red_count
        && handful.green_count  <= max_color_counts.green_count 
        && handful.blue_count   <= max_color_counts.blue_count
}

fn parse_line(line: &str) -> Result<Game, String> {
    let (id_part, handfuls_part) = line
        .split_once(": ").ok_or("Line has no ': ' seperator!")?;
    let id = id_part
        .strip_prefix("Game ").ok_or("Line does not start with 'Game '!")?
        .parse::<usize>().map_err(|_err| "Could not parse game id!")?;
    Ok(Game {id, handfuls: parse_handfuls(handfuls_part)?})
}

fn parse_handfuls(s: &str) -> Result<Vec<Handful>, String> {
    let mut handfuls = Vec::new();
    for handful_str in s.split("; ").collect::<Vec<&str>>() {
        handfuls.push(parse_handful(handful_str)?);
    }
    Ok(handfuls)
}

fn parse_handful(s: &str) -> Result<Handful, String> {
    let mut handful = Handful {
        red_count: 0,
        green_count: 0,
        blue_count: 0,
    };
    for part in s.split(", ").collect::<Vec<&str>>() {
        if part.ends_with(" red") {
            handful.red_count += part
                .strip_suffix(" red").ok_or("Could not parse red count")?
                .parse::<usize>().map_err(|_err| "Could not parse red count")?;
        } else if part.ends_with(" green") {
            handful.green_count += part
                .strip_suffix(" green").ok_or("Could not parse green count")?
                .parse::<usize>().map_err(|_err| "Could not parse green count")?;
        } else if part.ends_with(" blue") {
            handful.blue_count += part
                .strip_suffix(" blue").ok_or("Could not parse blue count")?
                .parse::<usize>().map_err(|_err| "Could not parse blue count")?;
        } else {
            return Err("Handful part must start with 'red ', 'green ' or 'blue '!".to_string());
        }
    } 
    Ok(handful)
}

#[cfg(test)]
mod tests {
    use crate::{parse_line, Game, Handful};

    #[test]
    fn test_parse_line() {
        assert_eq!(
            parse_line("Game 6: 3 red, 2 blue; 6 green, 13 blue; 11 blue, 1 red; 4 green, 3 red, 5 blue"),
            Ok(Game {
                id: 6,
                handfuls: vec![
                    Handful {red_count: 3, green_count: 0, blue_count: 2},
                    Handful {red_count: 0, green_count: 6, blue_count: 13},
                    Handful {red_count: 1, green_count: 0, blue_count: 11},
                    Handful {red_count: 3, green_count: 4, blue_count: 5},
                ]
            })
        );
    }
}
