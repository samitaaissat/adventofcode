use crate::custom_error::AocError;

#[tracing::instrument]
pub fn main() -> miette::Result<()> {
    let input = include_str!("../input1.txt");
    let result = process(input)?;
    println!("result: {}", result);
    Ok(())
}

#[tracing::instrument]
pub fn process(
    input: &str,
) -> miette::Result<u32, AocError> {
    let mut sum = 0;
    for line in input.lines() {
        sum += process_line(line)?;
    }
    Ok(sum)
}

#[tracing::instrument]
pub fn process_line(
    line: &str,
) -> miette::Result<u32, AocError> {
    let mut first: Option<u32> = None;
    let mut last: Option<u32> = None;
    for c in line.chars() {
        if c.is_ascii_digit() {
            let digit = c.to_digit(10).ok_or(AocError::ParseError)?;
            if first.is_none() {
                first = Some(digit);
            }
            last = Some(digit);
        }
    }

    return Ok(first.unwrap_or(0)*10+last.unwrap_or(first.unwrap_or(0)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet";
        assert_eq!(142, process(input)?);
        Ok(())
    }

    #[test]
    fn test_process_line() -> miette::Result<()> {
        assert_eq!(12, process_line("1abc2")?);
        assert_eq!(38, process_line("pqr3stu8vwx")?);
        assert_eq!(15, process_line("a1b2c3d4e5f")?);
        assert_eq!(77, process_line("treb7uchet")?);
        Ok(())
    }
}
