use std::{
    env,
    io::{self, Write as _},
};

fn main() -> io::Result<()> {
    let shifted_arguments = env::args()
        .skip(1) // First argument is usually executable path or name. We don't need it.
        .map(|string| shift_characters(&string))
        .collect::<Vec<_>>()
        .join(" ");
    let byte_slice = shifted_arguments.as_bytes();

    io::stdout().write_all(byte_slice)?;

    Ok(())
}

fn shift_characters(argument: &str) -> String {
    argument.chars().map(shift_character).collect()
}

#[expect(clippy::too_many_lines, reason = "52 characters makes 52 lines")]
const fn shift_character(char: char) -> char {
    match char {
        'a' => 'e',
        'e' => 'i',
        'i' => 'o',
        'o' => 'u',
        'u' => 'a',
        'A' => 'E',
        'E' => 'I',
        'I' => 'O',
        'O' => 'U',
        'U' => 'A',
        'b' => 'c',
        'c' => 'd',
        'd' => 'f',
        'f' => 'g',
        'g' => 'h',
        'h' => 'j',
        'j' => 'k',
        'k' => 'l',
        'l' => 'm',
        'm' => 'n',
        'n' => 'p',
        'p' => 'q',
        'q' => 'r',
        'r' => 's',
        's' => 't',
        't' => 'v',
        'v' => 'w',
        'w' => 'x',
        'x' => 'y',
        'y' => 'z',
        'z' => 'b',
        'B' => 'C',
        'C' => 'D',
        'D' => 'F',
        'F' => 'G',
        'G' => 'H',
        'H' => 'J',
        'J' => 'K',
        'K' => 'L',
        'L' => 'M',
        'M' => 'N',
        'N' => 'P',
        'P' => 'Q',
        'Q' => 'R',
        'R' => 'S',
        'S' => 'T',
        'T' => 'V',
        'V' => 'W',
        'W' => 'X',
        'X' => 'Y',
        'Y' => 'Z',
        'Z' => 'B',
        _ => char,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn alpha_characters_shifted_by_one() {
        assert_eq!(shift_characters("Hello, Zuckerberg."), "Jimmu, Badliscish.");
    }

    #[test]
    fn non_alpha_characters_not_shifted() {
        assert_eq!(shift_characters("123, !@#."), "123, !@#.");
    }
}
