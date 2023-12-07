use std::{fs::File, io::Read};

#[derive(Debug)]
struct Range {
    src_start: usize,
    dst_start: usize,
    size: usize,
}

type Map_ = Vec<Range>;

#[derive(Debug)]
struct ParseResult {
    seeds: Vec<usize>,
    maps: Vec<Map_>,
}

fn main() -> Result<(), String> {
    let mut file = File::open("input.txt").map_err(|err| err.to_string())?;
    let mut input = String::new();
    let _ = file.read_to_string(&mut input);
    println!(
        "Lowest location number: {}", 
        find_lowest_location_number(&input).expect("Failed to parse input!")
    );

    Ok(())
}

fn find_lowest_location_number(input: &str) -> Option<usize> {
    let parse_res = parse(input)?;
    parse_res.seeds.iter()
        .map(|seed| apply_maps(&parse_res.maps, *seed))
        .min()
}

fn parse(input: &str) -> Option<ParseResult> {
    let mut blocks = input.split("\n\n");
    let seeds = parse_nums(blocks.next()?.strip_prefix("seeds:")?)?;
    let mut maps = Vec::<Map_>::new();
    for block in blocks {
        let mut lines = block.split("\n");
        // Remove map title line.
        lines.next()?;
        let mut map_: Map_ = Vec::new();
        for range_line in lines {
            if range_line == "" {
                continue
            }
            let nums = parse_nums(range_line)?;
            map_.push(Range {
                dst_start: *nums.get(0)?, 
                src_start: *nums.get(1)?, 
                size: *nums.get(2)?
            })
        }
        maps.push(map_);
    }
    return Some(ParseResult {seeds, maps});
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

fn apply_maps(maps: &Vec<Map_>, src: usize) -> usize {
    maps.iter().fold(src, |src, map_| apply_map(map_, src))
}

fn apply_map(map_: &Map_, src: usize) -> usize {
    for range in map_ {
        if range.src_start <= src && src <= range.src_start + range.size {
            return (src - range.src_start) + range.dst_start;
        }
    }
    src
}

#[cfg(test)]
mod tests {
    use crate::find_lowest_location_number;

    #[test]
    fn test_find_lowest_location_number() {
        let input = [
            "seeds: 79 14 55 13"          , 
            ""                            , 
            "seed-to-soil map:"           , 
            "50 98 2"                     , 
            "52 50 48"                    , 
            ""                            , 
            "soil-to-fertilizer map:"     , 
            "0 15 37"                     , 
            "37 52 2"                     , 
            "39 0 15"                     , 
            ""                            , 
            "fertilizer-to-water map:"    , 
            "49 53 8"                     , 
            "0 11 42"                     , 
            "42 0 7"                      , 
            "57 7 4"                      , 
            ""                            , 
            "water-to-light map:"         , 
            "88 18 7"                     , 
            "18 25 70"                    , 
            ""                            , 
            "light-to-temperature map:"   , 
            "45 77 23"                    , 
            "81 45 19"                    , 
            "68 64 13"                    , 
            ""                            , 
            "temperature-to-humidity map:", 
            "0 69 1", 
            "1 0 69", 
            "", 
            "humidity-to-location map:", 
            "60 56 37", 
            "56 93 4", 
        ].join("\n");
        assert_eq!(
            find_lowest_location_number(&input),
            Some(35),
        );
    }
}
