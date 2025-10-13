// In this exercise, you'll learn some of the unique advantages that iterators
// can offer.

// TODO: Complete the `capitalize_first` function.
// "hello" -> "Hello"
fn capitalize_first(input: &str) -> String {
    let mut chars = input.chars();
    let mut capitalized: String = match chars.next() {
        None => String::new(),
        Some(first) => {
            first.to_uppercase().to_string()
        },
    };
    if input.is_empty() {
        return capitalized;
    }
    let non_start_chars = input.split_at(1);
    capitalized.push_str(non_start_chars.1);
    capitalized
}

// TODO: Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    let mut new_words: Vec<String> = vec![];
    for word in words {
        let word = capitalize_first(word);
        new_words.push(word.to_string());
    }
    new_words
}

// TODO: Apply the `capitalize_first` function again to a slice of string
// slices. Return a single string.
// ["hello", " ", "world"] -> "Hello World"
fn capitalize_words_string(words: &[&str]) -> String {
    let mut new_words: String = String::new();
    for word in words {
        let word = capitalize_first(word);
        new_words.push_str(word.as_str());
    }
    new_words
}

fn main() {
    let letter = capitalize_first("hello");
    println!("{letter}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
