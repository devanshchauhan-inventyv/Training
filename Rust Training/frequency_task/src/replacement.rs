//replacement.rs
/// Replaces a target character with a replacement character in the given string.
///
pub fn replace_char_in_string(
    line: &mut String,
    target_char: char,
    replacement_char: char,
) -> String {
    let mut vec_of_line: Vec<char> = line.chars().collect();

    for iter in vec_of_line.iter_mut() {
        if *iter == target_char {
            *iter = replacement_char;
        }
    }

    vec_of_line.into_iter().collect()
}
/// Replaces underscores in the given string with matched letter frequencies.
/// 
pub fn replace_underscore_with_matched_frequency(
    line: &mut String,
    target: char,
    mut matched_letters_frequency: [usize; 26],
) -> String {
    let mut vec_of_line: Vec<char> = line.chars().collect();

    for i in vec_of_line.iter_mut() {
        if *i == target {
            let replace_by_letter: char;
            for (letter, frequency) in matched_letters_frequency.iter_mut().enumerate() {
                if *frequency > 0 {
                    replace_by_letter = (letter as u8 + b'a') as char;
                    *frequency -= 1;
                    *i = replace_by_letter;
                    break;
                }
            }
        }
    }

    vec_of_line.into_iter().collect()
}
