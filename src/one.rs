use regex::Regex;

pub fn solve_part_one(input: &str) -> i32 {
    let non_digits = Regex::new("\\D").expect("Invalid regex pattern!");
    let digits: Vec<_> = input.lines()
        .map(|line| {
            non_digits.replace_all(line, "")
        }).map(|only_digits| {
        let mut first = only_digits.chars().nth(0).unwrap().to_string();
        let last = only_digits.chars().last().unwrap().to_string();
        first.push_str(last.as_ref());
        first
    }).map(|num_string| {
        num_string.parse::<i32>()
    }).filter(|opt| opt.is_ok())
        .map(|ok| ok.unwrap())
        .collect();
    digits.iter().sum()
}

pub fn solve_part_two(input: &str) -> i32 {
    // convert to part one syntax
    let converted = input
        .replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
    solve_part_one(converted.as_str())
}

fn replace_written_out_digits(input: &str) -> &str {
    let matches = input.matches("(one|two|three|four|five|six|seven|eight|nine|\\d)");
    ""
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn solves_part_one() {
        let input = r#"1abc2
            pqr3stu8vwx
            a1b2c3d4e5f
            treb7uchet"#;

        let actual = solve_part_one(input);

        assert_eq!(142, actual);
    }

    #[test]
    fn solves_part_two() {
        let input = r#"two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen"#;


        let actual = solve_part_two(input);

        assert_eq!(281, actual);
    }

}
