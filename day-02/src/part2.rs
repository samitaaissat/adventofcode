use crate::custom_error::AocError;
use std::collections::HashMap;

#[tracing::instrument]
pub fn process(input: &str) -> Result<i64, AocError> {
    let mut sum_of_powers = 0;

    for line in input.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let subsets = parts[1].split("; ");

        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;

        for subset in subsets {
            let mut counts = HashMap::new();
            for cube_info in subset.split(", ") {
                let cube_parts: Vec<&str> = cube_info.split_whitespace().collect();
                let count: i32 = cube_parts[0].parse().unwrap();
                let color = cube_parts[1];

                *counts.entry(color).or_insert(0) += count;
            }

            max_red = max_red.max(*counts.get("red").unwrap_or(&0));
            max_green = max_green.max(*counts.get("green").unwrap_or(&0));
            max_blue = max_blue.max(*counts.get("blue").unwrap_or(&0));
        }

        let power = max_red as i64 * max_green as i64 * max_blue as i64;
        sum_of_powers += power;
    }

    Ok(sum_of_powers)
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
        assert_eq!(2286, process(input)?);
        Ok(())
    }
}
