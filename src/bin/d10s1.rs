use aoc2021::prelude::*;

fn parse_one(s: &str, pos: usize) -> Result<(&str, usize), usize> {
    if let Some(ch) = s.chars().next() {
        let expected = match ch {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => return Err(pos),
        };

        parse_multi(&s[1..], pos + 1, Some(expected))
    } else {
        Ok((s, pos))
    }
}

fn parse_multi(
    mut s: &str,
    mut pos: usize,
    expected: Option<char>,
) -> Result<(&str, usize), usize> {
    while let Some(next) = s.chars().next() {
        match (next, expected) {
            (x, Some(y)) if x == y => {
                s = &s[1..];
                pos += 1;
                break;
            }
            ('(' | '[' | '{' | '<', _) => {
                let (new_s, new_pos) = parse_one(s, pos)?;
                s = new_s;
                pos = new_pos;
            }
            _ => return Err(pos),
        }
    }

    Ok((s, pos))
}

fn main() {
    let lines = parsed_lines::<String>();
    let mut score = 0;

    for line in lines {
        if let Err(x) = parse_multi(&line, 0, None) {
            match line.chars().nth(x) {
                Some(')') => score += 3,
                Some(']') => score += 57,
                Some('}') => score += 1197,
                Some('>') => score += 25137,
                _ => unreachable!(),
            }
        }
    }

    println!("{}", score);
}
