pub fn generate_ngrams(tokens: &[String], n: usize) -> Vec<Vec<String>> {
    if n == 0 || tokens.len() < n {
        return vec![];
    }

    tokens
        .windows(n)
        .map(|window| window.iter().cloned().collect())
        .collect()
}
