//! Run this file with `cargo test --test 05_split_items`.

//! TODO: Implement a struct called `SplitItems`, which will receive a string slice and a delimiter
//! char in its constructor.
//!
//! The struct should act as an iterator which iterates over all substrings of the input, separated
//! by the delimiter. The iterator should never return an empty string; it should automatically skip
//! over empty strings.
//!
//! The iterator has to be **lazy**! It should not copy the whole input array
//! (in other words, it should have space complexity O(1)).

struct SplitItems<'a> {
    data: &'a str,
    delimiter: char,
    start: usize,
    end: usize,
}

impl<'a> SplitItems<'a> {
    const fn new(data: &'a str, delimiter: char) -> Self {
        Self {
            data,
            delimiter,
            start: 0,
            end: 0,
        }
    }
}

impl<'a> Iterator for SplitItems<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.end == self.data.len() {
            return None;
        }

        self.start = self.end;
        while self.start < self.data.len()
            && self.data.chars().nth(self.start).unwrap() == self.delimiter
        {
            self.start += 1;
        }

        self.end = self.start;
        while self.end < self.data.len()
            && self.data.chars().nth(self.end).unwrap() != self.delimiter
        {
            self.end += 1;
        }

        if self.start == self.end {
            return self.next();
        }

        Some(&self.data[self.start..self.end])
    }
}

/// Below you can find a set of unit tests.
#[cfg(test)]
mod tests {
    use crate::SplitItems;

    #[test]
    fn split_empty() {
        assert!(SplitItems::new("", ' ').next().is_none());
    }

    #[test]
    fn split_one_delimiter() {
        assert!(SplitItems::new("c", 'c').next().is_none());
    }

    #[test]
    fn split_only_delimiters() {
        assert!(SplitItems::new("ccc", 'c').next().is_none());
    }

    #[test]
    fn split_delimiter_at_beginning() {
        let result = SplitItems::new(" asd", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_delimiters_at_beginning() {
        let result = SplitItems::new("  asd", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_delimiter_at_end() {
        let result = SplitItems::new("asd ", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_delimiters_at_end() {
        let result = SplitItems::new("asd  ", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["asd"]);
    }

    #[test]
    fn split_single_chars() {
        let result = SplitItems::new("a b c d e", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["a", "b", "c", "d", "e"]);
    }

    #[test]
    fn split_complex() {
        let result = SplitItems::new("   abc   bde casdqw dee xe ", ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["abc", "bde", "casdqw", "dee", "xe"]);
    }

    #[test]
    fn split_complex_custom_delimiter() {
        let result = SplitItems::new("pppabcpppbdepcasdqwpdeepxep", 'p').collect::<Vec<_>>();
        assert_eq!(result, vec!["abc", "bde", "casdqw", "dee", "xe"]);
    }

    #[test]
    fn split_check_reference() {
        let data = String::from("foo bar");
        let result = SplitItems::new(&data, ' ').collect::<Vec<_>>();
        assert_eq!(result, vec!["foo", "bar"]);
    }

    #[test]
    fn split_check_type() {
        let result: SplitItems<'_> = SplitItems::new("foo bar baz", ' ');
        assert_eq!(result.collect::<Vec<_>>(), vec!["foo", "bar", "baz"]);
    }
}
