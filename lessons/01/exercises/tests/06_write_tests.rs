//! Run this file with `cargo test --test 06_write_tests`.

/// This function implements a string sanitization logic that should uphold the following
/// properties:
/// - After sanitization, the result must not end with the character `x`
/// - After sanitization, the result must not end with the character `o`
/// - After sanitization, the result must not end with the string `.exe`
///
/// The function assumes that the input to the function only consists of lower and uppercase
/// characters from the English alphabet and digits 0-9.
///
/// The implementation contains some bugs.
///
/// Your task is to write a set (at least 8) of unit tests, use them to find (at least 2) bugs in
/// this function and then fix the function.
fn sanitize(mut input: &str) -> &str {
    loop {
        if input.ends_with("x") {
            input = input.trim_end_matches("x");
        } else if input.ends_with("o") {
            input = input.trim_end_matches("o");
        } else if input.ends_with(".exe") {
            input = &input[0..input.len() - 4];
        } else {
            break;
        }
    }
    input
}

/// TODO: write tests for the `sanitize` function
///
/// Bonus: can you find any bugs using the [proptest](https://proptest-rs.github.io/proptest/intro.html)
/// crate?
/// Note that you will probably need to run `cargo test` with the `PROPTEST_DISABLE_FAILURE_PERSISTENCE=1`
/// environment variable to make proptest work for tests stored in the `tests` directory.
#[cfg(test)]
mod tests {
    use super::sanitize;

    #[test]
    fn test_empty_string() {
        assert_eq!(sanitize(""), "");
    }

    #[test]
    fn test_nothing_to_remove() {
        assert_eq!(sanitize("helle"), "helle");
        assert_eq!(sanitize("hello.exew"), "hello.exew");
    }

    #[test]
    fn test_remove_x() {
        assert_eq!(sanitize("hellox"), "hell");
    }

    #[test]
    fn test_remove_o() {
        assert_eq!(sanitize("helloo"), "hell");
    }

    #[test]
    fn test_remove_exe() {
        assert_eq!(sanitize("hello.exe"), "hell");
    }

    #[test]
    fn test_remove_xo() {
        assert_eq!(sanitize("helloxo"), "hell");
    }

    #[test]
    fn test_remove_x_exe() {
        assert_eq!(sanitize("hellox.exe"), "hell");
    }

    #[test]
    fn test_remove_o_exe() {
        assert_eq!(sanitize("helloo.exe"), "hell");
    }

    #[test]
    fn test_remove_xo_exe() {
        assert_eq!(sanitize("helloxo.exe"), "hell");
    }
}
