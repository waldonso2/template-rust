//! PLACEHOLDER: what the tool does, in one line. The logic lives here, without input/output, so the tests and the
//! performance report call it directly.

use std::collections::HashMap;

/// PLACEHOLDER example: the words of `text` by frequency, most frequent first (then alphabetically).
#[must_use]
pub fn word_counts(text: &str) -> Vec<(String, usize)> {
    let mut counts: HashMap<String, usize> = HashMap::new();
    for raw in text.split_whitespace() {
        let word = raw.trim_matches(|c: char| ".,;:!?\"'()".contains(c)).to_lowercase();
        if !word.is_empty() {
            *counts.entry(word).or_insert(0) += 1;
        }
    }
    let mut sorted: Vec<(String, usize)> = counts.into_iter().collect();
    sorted.sort_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)));
    sorted
}

#[cfg(test)]
mod tests {
    use super::word_counts;

    #[test]
    fn counts_by_frequency_then_alphabetically() {
        let expected: Vec<(String, usize)> = vec![("b".into(), 3), ("a".into(), 2), ("c".into(), 1)];
        assert_eq!(word_counts("b a, B! c a b"), expected);
    }

    #[test]
    fn punctuation_only_is_no_word() {
        assert_eq!(word_counts("... !!"), Vec::<(String, usize)>::new());
    }
}
