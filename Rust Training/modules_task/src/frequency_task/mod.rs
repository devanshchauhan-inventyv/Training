//frequency_task.rs
//!This module contains functionality for the frequency task
///
///This module uses the child module frequency to use function of that module
///
mod frequency;
///This module uses the child module replacement to use function of that module
///
mod replacement;
use frequency::*;
use replacement::*;

/// This function demonstrates a frequency processing task.
///
/// It performs the following steps:
/// 1. Replaces a target character with a replacement character in a given string.
/// 2. Calculates letter frequencies for two strings.
/// 3. Merges the frequencies of both strings.
/// 4. Identifies and prints the matched letters and their frequencies.
/// 5. Identifies and prints the left out letters and their frequencies.
/// 6. Replaces underscores in the result string with the matched letter frequencies.
///
pub fn process_frequency_task() {
    // Step 1: Replace a target character with a replacement character in a given string
    let mut line = String::from("Welcome to Inventyv software");
    let target_char = 'e';
    let replacement_char = '_';
    let mut result = replace_char_in_string(&mut line, target_char, replacement_char);
    println!("{}", result);

    // Step 2: Calculate letter frequencies for two strings
    let str1 = String::from("lorem ipsum");
    let str2 = String::from("ceaser cypher");
    let frequency_of_str1 = calculate_letter_frequencies(&str1);
    let frequency_of_str2 = calculate_letter_frequencies(&str2);

    // Step 3: Merge the frequencies of both strings
    let mut left_out_letters: [usize; 26] = [0; 26];
    let mut matched_letters_frequency =
        merge_both_frequencies(frequency_of_str1, frequency_of_str2);

    // Step 4: Identify and print matched letters and their frequencies
    let looping_string = shorter_string(&str1, &str2);
    calculate_letter_present_in_both_string(
        &str1,
        &str2,
        looping_string,
        &mut matched_letters_frequency,
        &mut left_out_letters,
    );

    println!("the matched letters and their frequency are ");
    print_frequencies(matched_letters_frequency);

    // Step 5: Identify and print left out letters and their frequencies
    println!("the left out letters and their frequency are ");
    print_frequencies(left_out_letters);

    // Step 6: Replace underscores in the result string with the matched letter frequencies
    let target = '_';
    let final_result2 =
        replace_underscore_with_matched_frequency(&mut result, target, matched_letters_frequency);
    println!("{}", final_result2);
}
