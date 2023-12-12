use crate::custom_error::AocError;
use rayon::prelude::*;

#[tracing::instrument]
pub fn main() -> miette::Result<()> {
    let input = include_str!("../input2.txt");
    let result = process(input)?;
    println!("result: {}", result);
    Ok(())
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let sum: Result<u32, _> = input
        .par_lines() // Use parallel iterator over the lines
        .map(|line| process_line(line)) // Process each line
        .try_fold(|| 0u32, |acc, line_result| {
            line_result.map(|value| acc + value)
        }) // Try to fold results into a single sum
        .try_reduce(|| 0u32, |a, b| Ok(a + b)); // Reduce results from different threads

    sum.map_err(|e| e.into())
}

#[tracing::instrument]
pub fn process_line(
    line: &str,
) -> miette::Result<u32, AocError> {
        let replacements = [
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9"),
    ];

    let mut new_line = String::new();

    for (i, c) in line.char_indices() {
        if c.is_ascii_alphabetic() {
            let mut replaced = false;
            for &(word, digit) in &replacements {
                if line[i..].starts_with(word) {
                    new_line.push_str(digit);
                    replaced = true;
                    break;
                }
            }
            if !replaced {
                new_line.push(c);
            }
        } else {
            new_line.push(c);
        }
    }

    let first_digit = new_line.chars().filter(|c| c.is_ascii_digit()).next().and_then(|c| c.to_digit(10)).unwrap_or(0);
    let last_digit = new_line.chars().filter(|c| c.is_ascii_digit()).last().and_then(|c| c.to_digit(10)).unwrap_or(first_digit);

    Ok(first_digit * 10 + last_digit)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen";
        assert_eq!(281, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_line() -> miette::Result<()> {
        assert_eq!(29, process_line("two1nine")?);
        assert_eq!(83, process_line("eightwothree")?);
        assert_eq!(13, process_line("abcone2threexyz")?);
        assert_eq!(24, process_line("xtwone3four")?);
        assert_eq!(42, process_line("4nineeightseven2")?);
        assert_eq!(14, process_line("zoneight234")?);
        assert_eq!(76, process_line("7pqrstsixteen")?);
        Ok(())
    }
}
