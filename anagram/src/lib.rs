use std::collections::HashSet;

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &[&'a str]) -> HashSet<&'a str> {
    // word will never be returned as a result, thus I will not add the lifetime 'a to it :)
    // Thus, you can drop that variable and still use the returned value from this function.
    let mut result = HashSet::new(); // Store found anagrams here.

    let lower_case_word: String = word.to_lowercase(); // Expensive method call.
    let mut word_as_vec = lower_case_word.chars().collect::<Vec<char>>(); // Make it a vector of chars
    word_as_vec.sort_unstable(); // So that we can sort it
    let word_sorted: String = word_as_vec.into_iter().collect(); // and make it a string, so we can compare it

    // Let's go through each candidate anagram in the list.
    for candidate in possible_anagrams {
        if *candidate.to_lowercase() == lower_case_word {
            // No exact matches. Skip this.
            continue;
        }
        let mut cand_as_vec: Vec<char> = candidate.to_lowercase().chars().collect(); // Make it a vector of chars
        cand_as_vec.sort_unstable(); // Sort it
        let cand_sorted: String = cand_as_vec.into_iter().collect(); // make it a string again

        if cand_sorted == word_sorted {
            // If they're the same, add the word!
            result.insert(*candidate);
        }
    }

    result // Return the final result.
}
