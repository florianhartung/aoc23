pub fn main() {
    let input = include_str!("../inputs/day1longer.txt");

    let calibration_value_sum: usize = input
        .lines()
        .flat_map(parse_line)
        .map(|(first, last)| first * 10 + last)
        .sum();

    println!("Day 1 Part 2: {calibration_value_sum}");
}

fn parse_line(line: &str) -> Option<(usize, usize)> {
    const DIGITS: [&str; 18] = [
        "1", "one", "2", "two", "3", "three", "4", "four", "5", "five", "6", "six", "7", "seven",
        "8", "eight", "9", "nine",
    ];

    let first_idx = DIGITS
        .iter()
        .enumerate()
        .flat_map(|(i, d)| line.find(d).map(|idx| (idx, i / 2 + 1)))
        .min_by_key(|(idx, _)| *idx);

    let last_idx = DIGITS
        .iter()
        .enumerate()
        .flat_map(|(i, d)| line.rfind(d).map(|idx| (idx, i / 2 + 1)))
        .max_by_key(|(idx, _)| *idx);

    (first_idx.or(last_idx).zip(last_idx.or(first_idx)))
        .map(|((_, first), (_, last))| (first, last))
}

#[cfg(test)]
mod tests {
    use crate::parse_line;

    #[test]
    pub fn single_line() {
        // Given examples
        assert_eq!(parse_line("two1nine"), Some((2, 9)));
        assert_eq!(parse_line("eightwothree"), Some((8, 3)));
        assert_eq!(parse_line("abcone2threexyz"), Some((1, 3)));
        assert_eq!(parse_line("xtwone3four"), Some((2, 4)));
        assert_eq!(parse_line("4nineeightseven2"), Some((4, 2)));
        assert_eq!(parse_line("zoneight234"), Some((1, 4)));
        assert_eq!(parse_line("7pqrstsixteen"), Some((7, 6)));

        // Some other special cases
        assert_eq!(parse_line("1"), Some((1, 1)));
        assert_eq!(parse_line("oneight"), Some((1, 8)));
        assert_eq!(parse_line("two"), Some((2, 2)));
    }
}
