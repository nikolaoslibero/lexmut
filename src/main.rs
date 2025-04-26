use std::{
    env,
    io::{self, Write as _},
};

fn main() -> io::Result<()> {
    let arguments_and_path = env::args().collect::<Vec<String>>();
    let Some(arguments) = arguments_and_path.get(1..) else {
        return Ok(());
    };
    let mut stdout = io::stdout();

    stdout.write_all(transform_arguments(arguments).join(" ").as_bytes())?;

    Ok(())
}

fn transform_arguments(strings: &[String]) -> Vec<String> {
    strings
        .iter()
        .map(|string| swap_characters(string))
        .collect::<Vec<String>>()
}

fn swap_characters(argument: &str) -> String {
    argument.chars().map(swap_character).collect::<String>()
}

#[expect(clippy::too_many_lines, reason = "52 characters makes 52 lines")]
const fn swap_character(char: char) -> char {
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
