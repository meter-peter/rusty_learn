use std::collections::HashMap;

// Calculate term frequency for a document
pub fn term_frequency(tokens: &[String]) -> HashMap<String, f64> {
    let mut tf = HashMap::new();
    let total_words = tokens.len() as f64;

    for token in tokens {
        *tf.entry(token.to_string()).or_insert(0.0) += 1.0;
    }

    for value in tf.values_mut() {
        *value /= total_words;
    }

    tf
}

// Calculate inverse document frequency for a collection of documents
pub fn inverse_document_frequency(documents: &[Vec<String>]) -> HashMap<String, f64> {
    let mut idf = HashMap::new();
    let total_documents = documents.len() as f64;

    for document in documents {
        let mut unique_terms = HashMap::new();
        for term in document {
            unique_terms.insert(term.to_string(), true);
        }

        for term in unique_terms.keys() {
            *idf.entry(term.to_string()).or_insert(0.0) += 1.0;
        }
    }

    for value in idf.values_mut() {
        *value = (*value / total_documents).ln();
    }

    idf
}

// Calculate TF-IDF for a document
pub fn tf_idf(document: &[String], documents: &[Vec<String>]) -> HashMap<String, f64> {
    let tf = term_frequency(document);
    let idf = inverse_document_frequency(documents);
    let mut tf_idf = HashMap::new();

    for (term, value) in &tf {
        if let Some(idf_value) = idf.get(term) {
            tf_idf.insert(term.to_string(), value * idf_value);
        }
    }

    tf_idf
}
