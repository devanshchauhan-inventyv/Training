//frequency.rs
/// #This module contains the function for frequency related processing for the frequency_task.
/// 
/// Calculates the letter frequencies for the given input string.
/// Returns an array representing the frequency of each letter in the alphabet.
pub fn calculate_letter_frequencies(input: &str) -> [usize; 26] {
  
    let mut frequencies = [0; 26];

    for char in input.chars() {
        if char.is_ascii_alphabetic() {
            let index = char.to_ascii_lowercase() as usize - 'a' as usize;
            frequencies[index] += 1;
        }
    }

    frequencies
}
/// Prints the letter frequencies in a readable format.
pub fn print_frequencies(frequencies: [usize; 26]) {
    for (letter, frequency) in frequencies.iter().enumerate() {
        if *frequency > 0 {
            let letter_char = (letter as u8 + b'a') as char;
            println!("the frequency of {}: {}", letter_char, frequency);
        }
    }
}
/// Merges the frequencies of two arrays into a new array.
/// This function is named "merge_both_frequencies".
pub fn merge_both_frequencies(freq1: [usize; 26], freq2: [usize; 26]) -> [usize; 26] {
    let mut merged_letters_frequency = [0; 26];
    for (i, (freq1, freq2)) in freq1.iter().zip(freq2.iter()).enumerate() {
        merged_letters_frequency[i] = freq1 + freq2;
    }
    merged_letters_frequency
}
/// Calculates letter presence in both strings, updating frequencies accordingly.
/// Uses additional information provided in `looping_string` and updates
/// `matched_letters_frequency` and `left_out_letters` arrays.
pub fn calculate_letter_present_in_both_string(
    str1: &str,
    str2: &str,
    looping_string: &String,
    matched_letters_frequency: &mut [usize; 26],
    left_out_letters: &mut [usize; 26],
) {
    for i in 0..looping_string.len() {
        let vec_of_string1: Vec<char> = str1.chars().collect();
        let vec_of_string2: Vec<char> = str2.chars().collect();
        let mut index;

        if !vec_of_string1.contains(&vec_of_string2[i]) {
            index = vec_of_string2[i].to_ascii_lowercase() as usize - 'a' as usize;
            matched_letters_frequency[index] = 0;
            left_out_letters[index] += 1;
        }
        if !vec_of_string2.contains(&vec_of_string1[i]) {
            index = vec_of_string1[i].to_ascii_lowercase() as usize - 'a' as usize;
            matched_letters_frequency[index] = 0;
            left_out_letters[index] += 1;
        }
    }
}
/// Returns the reference to the shorter of the two input strings which is used as a looping string in the merge_both_frequencies function
pub fn shorter_string<'a>(str1: &'a String, str2: &'a String) -> &'a String {
    if str1.len() > str2.len() {
        str2
    } else {
        str1
    }
}
