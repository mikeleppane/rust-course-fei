//! Run this file with `cargo test --test 04_luhn_algorithm`.

// TODO: Implement the Luhn algorithm (https://en.wikipedia.org/wiki/Luhn_algorithm),
// which is used to check the validity of e.g. bank or credit card numbers.

const fn luhn_algorithm(mut number: u64) -> bool {
    let mut sum = 0;
    let mut is_second = false;
    while number > 0 {
        let digit = number % 10;
        if is_second {
            let mut doubled = digit * 2;
            if doubled > 9 {
                doubled -= 9;
            }
            sum += doubled;
        } else {
            sum += digit;
        }
        is_second = !is_second;
        number /= 10;
    }
    sum % 10 == 0
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use super::luhn_algorithm;

    #[test]
    fn luhn_zero() {
        assert!(luhn_algorithm(0));
    }

    #[test]
    fn luhn_small_correct() {
        assert!(luhn_algorithm(18));
    }

    #[test]
    fn luhn_small_incorrect() {
        assert!(!luhn_algorithm(10));
    }

    #[test]
    fn luhn_correct() {
        assert!(luhn_algorithm(17_893_729_974));
        assert!(luhn_algorithm(79_927_398_713));
    }

    #[test]
    fn luhn_incorrect() {
        assert!(!luhn_algorithm(17_893_729_975));
        assert!(!luhn_algorithm(17_893_729_976));
        assert!(!luhn_algorithm(17_893_729_977));
        assert!(!luhn_algorithm(123_456));
    }
}
