use std::{iter::Peekable, str::Lines};

pub fn skip_comments_and_whitespace(lines: &mut Peekable<Lines>) {
    while let Some(line) = lines.peek() {
        if line.char_indices().count() > 1 {
            match &line[0..1] {
                "#" | "\0" | "\n" => {
                    lines.next();
                    continue;
                }
                _ => break,
            }
        } else {
            lines.next();
            continue;
        }
    }
}

pub fn split_line_into_u8(line: &str) -> Vec<u8> {
    line.split(" ")
        .into_iter()
        .map(|f| match f.parse::<u8>() {
            Ok(w) => w,
            Err(_) => panic!("Parse error - expected unsigned integer on line"),
        })
        .collect()
}