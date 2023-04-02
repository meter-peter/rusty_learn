use regex::Regex;
use unicode_segmentation::UnicodeSegmentation;

// Tokenize text into words using regular expressions
pub fn word_tokenize(text: &str) -> Vec<String> {
    let re = Regex::new(r"\b\w+\b").unwrap();
    re.find_iter(text)
        .map(|word| word.as_str().to_string())
        .collect()
}

// Tokenize text into sentences
pub fn sentence_tokenize(text: &str) -> Vec<String> {
    UnicodeSegmentation::sentences(text)
        .map(|sentence| sentence.to_string())
        .collect()
}
//Tokenize whitespace
pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|word| word.to_string())
        .collect()
}

