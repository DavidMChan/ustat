// Copyright (c) 2021 David Chan
//
// This software is released under the MIT License.
// https://opensource.org/licenses/MIT

pub fn parse_delimiter_from_string(delimiter: String) -> char {
    let mut reader = delimiter.chars();
    let escape_char = reader
        .next()
        .expect("Need to pass a character as a delimiter.");
    if escape_char != '\\' {
        return escape_char;
    }
    match reader
        .next()
        .expect("\\ escaped delimiters must be exactly two characters.")
    {
        't' => '\t',
        'n' => '\n',
        'r' => '\r',
        '\\' => '\\',
        '0' => '\0',
        _ => panic!("Unsupported escaped delimiter character."),
    }
}
