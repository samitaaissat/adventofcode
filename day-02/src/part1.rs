use crate::custom_error::AocError;
use std::collections::HashMap;

pub fn main() -> Result<(), AocError> {
    let input = include_str!("../input1.txt");
    let result = process(input)?;
    println!("result: {}", result);
    Ok(())
}

#[tracing::instrument]
pub fn process(input: &str) -> Result<i32, AocError> {
    let mut sum_of_ids = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let game_id: i32 = parts[0].split_whitespace().nth(1).unwrap().parse().unwrap();
        let subsets = parts[1].split("; ");

        let mut possible = true;
        for subset in subsets {
            let mut counts = HashMap::new();
            for cube_info in subset.split(", ") {
                let cube_parts: Vec<&str> = cube_info.split_whitespace().collect();
                let count: i32 = cube_parts[0].parse().unwrap();
                let color = cube_parts[1];

                *counts.entry(color).or_insert(0) += count;
            }

            if *counts.get("red").unwrap_or(&0) > 12
                || *counts.get("green").unwrap_or(&0) > 13
                || *counts.get("blue").unwrap_or(&0) > 14
            {
                possible = false;
                break;
            }
        }

        if possible {
            sum_of_ids += game_id;
        }
    }

    Ok(sum_of_ids)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(8, process(input)?);
        Ok(())
    }
}
