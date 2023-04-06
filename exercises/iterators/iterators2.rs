// iterators2.rs
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a hint.


// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"

// The variable `first` is a `char`. It needs to be capitalized and added to the
// remaining characters in `c` in order to return the correct `String`.
// The remaining characters in `c` can be viewed as a string slice using the
// `as_str` method.
// The documentation for `char` contains many useful methods.
// https://doc.rust-lang.org/std/primitive.char.html

pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => {
            let mut c_upper = first.to_uppercase().to_string();
            c_upper.push_str(c.as_str());
            c_upper
        }
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]

// Create an iterator from the slice. Transform the iterated values by applying
// the `capitalize_first` function. Remember to collect the iterator.

pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    words.iter().map(|s| capitalize_first(s)).collect()

}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"

// This is surprising similar to the previous solution. Collect is very powerful
// and very general. Rust just needs to know the desired type.

pub fn capitalize_words_string(words: &[&str]) -> String {
    words.iter().map(|s| capitalize_first(s)).collect()
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
