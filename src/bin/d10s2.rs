use aoc2021::prelude::*;

fn parse_one(
    s: &str,
    pos: usize,
    autocomplete: Vec<char>,
) -> Result<(&str, usize, Vec<char>), usize> {
    if let Some(ch) = s.chars().next() {
        let expected = match ch {
            '(' => ')',
            '[' => ']',
            '{' => '}',
            '<' => '>',
            _ => return Err(pos),
        };

        parse_multi(&s[1..], pos + 1, autocomplete, Some(expected))
    } else {
        Ok((s, pos, autocomplete))
    }
}

fn parse_multi(
    mut s: &str,
    mut pos: usize,
    mut autocomplete: Vec<char>,
    expected: Option<char>,
) -> Result<(&str, usize, Vec<char>), usize> {
    loop {
        match (s.chars().next(), expected) {
            (None, Some(missing)) => {
                autocomplete.push(missing);
                break;
            }
            (None, None) => break,
            (Some(x), Some(y)) if x == y => {
                s = &s[1..];
                pos += 1;
                break;
            }
            (Some('(' | '[' | '{' | '<'), _) => {
                let (new_s, new_pos, new_autocomplete) = parse_one(s, pos, autocomplete)?;
                s = new_s;
                pos = new_pos;
                autocomplete = new_autocomplete;
            }
            _ => return Err(pos),
        }
    }

    Ok((s, pos, autocomplete))
}

fn main() {
    let lines = parsed_lines::<String>();
    let mut scores = Vec::new();

    for line in lines {
        if let Ok((_, _, x)) = parse_multi(&line, 0, Vec::new(), None) {
            let mut score = 0u128;

            for ch in x {
                score *= 5;

                score += match ch {
                    ')' => 1,
                    ']' => 2,
                    '}' => 3,
                    '>' => 4,
                    _ => unreachable!(),
                };
            }

            scores.push(score);
        }
    }

    scores.sort_unstable();

    println!("{}", scores[scores.len() / 2]);
}
