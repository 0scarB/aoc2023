use std::{fs::File, io::Read};

struct Card {
    copies: usize,
    winning_nums: Vec<usize>,
    our_nums: Vec<usize>,
}

fn main() -> Result<(), String> {
    let mut file = File::open("input.txt").map_err(|err| err.to_string())?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    println!(
        "Total number of cards: {}", 
        count_total_cards(&input).expect("Failed to parse input!")
    );

    Ok(())
}

fn count_total_cards(input: &str) -> Option<usize> {
    let mut count = 0;
    let mut cards = parse(input)?;
    for i in 0..cards.len() {
        let card = cards.get(i)?;
        let copies = card.copies;
        count += copies;
        let winning_nums_count = count_winning_nums(&card);
        for j in i+1..i+winning_nums_count+1 {
            if let Some(future_card) = cards.get_mut(j) {
                future_card.copies += copies;
            }
        }
    }
    return Some(count)
}

fn parse(input: &str) -> Option<Vec<Card>> {
    input.lines().map(|l| {
        let (_, nums_part) = l.split_once(":")?;
        let (winning_nums_part, our_nums_part) = nums_part.split_once("|")?;
        let winning_nums = parse_nums(winning_nums_part)?;
        let our_nums = parse_nums(our_nums_part)?;
        Some(Card {copies: 1, winning_nums, our_nums})
    }).try_fold(Vec::<Card>::new(), |mut v, opt_card| {
        opt_card.map(|card| {
            v.push(card);
            v
        })
    })
}

fn parse_nums(s: &str) -> Option<Vec<usize>> {
    s.split(" ").filter(|s| *s != "").map(|s| s.trim().parse::<usize>())
        .try_fold(Vec::<usize>::new(), |mut v, num_res| {
            num_res.map_or(None, |num| {
                v.push(num);
                Some(v)
            })
        })
}

fn count_winning_nums(card: &Card) -> usize {
    let mut count = 0;
    for our_num in &card.our_nums {
        for winning_num in &card.winning_nums {
            if our_num == winning_num {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use crate::{count_total_cards};

    #[test]
    fn test_score_input() {
        let input = [
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ].join("\n");
        assert_eq!(
            count_total_cards(&input),
            Some(30),
        );
    }
}
