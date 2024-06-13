use regex::Regex;

pub fn split(input: &str, min_length: usize) -> Vec<&str> {
    let sentence_delimiters = {
        let mut delimiters = [r"\.", "!", r"\?", r"\.\.\.", ":"];
        delimiters.sort_by_key(|p| std::cmp::Reverse(p.len()));
        let combined_pattern = delimiters.join("|");
        Regex::new(&combined_pattern).unwrap()
    };
    let phrase_delimiters = {
        let mut delimiters = [r"\.", ";", "!", r"\?", r"\.\.\.", ",", ":", "-"];
        delimiters.sort_by_key(|p| std::cmp::Reverse(p.len()));
        let combined_pattern = delimiters.join("|");
        Regex::new(&combined_pattern).unwrap()
    };

    SentenceIterator::from(input, min_length, sentence_delimiters)
        .flat_map(|s| -> Vec<&str> {
            SentenceIterator::from(s, min_length, phrase_delimiters.clone()).collect()
        })
        .collect()
}

struct SentenceIterator<'a> {
    rest: &'a str,
    min_length: usize,
    delimiter_regex: Regex,
}
impl<'a> SentenceIterator<'a> {
    fn from(source: &'a str, min_length: usize, delimiter_regex: Regex) -> Self {
        Self {
            rest: source,
            min_length,
            delimiter_regex,
        }
    }
}
impl<'a> Iterator for SentenceIterator<'a> {
    type Item = &'a str;

    fn next(&mut self) -> Option<Self::Item> {
        if self.rest.is_empty() {
            return None;
        }
        let mut start = 0;
        loop {
            match self.delimiter_regex.find_at(self.rest, start) {
                Some(m) if m.end() < self.min_length => {
                    start = m.end();
                    continue;
                }
                Some(m) => {
                    let next_rest = &self.rest[m.end()..];
                    if next_rest.len() < self.min_length {
                        let last = self.rest;
                        self.rest = "";
                        return Some(last);
                    }
                    let found = &self.rest[..m.end()];
                    self.rest = next_rest;
                    return Some(found);
                }
                None => {
                    let last = self.rest;
                    self.rest = "";
                    return Some(last);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::split;
    use pretty_assertions::assert_eq;

    #[test]
    fn test_a() {
        let min_length = 30;
        let input = "Mayor, we need to talk about what's happening in this town. It's like... the fabric of reality is unraveling. I've seen animals acting strangely, and buildings shifting like they're made of sand. I'm really scared, to be honest.";
        let expected = vec![
            "Mayor, we need to talk about what's happening in this town.",
            " It's like... the fabric of reality is unraveling.",
            " I've seen animals acting strangely,",
            " and buildings shifting like they're made of sand.",
            " I'm really scared, to be honest.",
        ];
        let actual = split(input, min_length);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_b() {
        let min_length = 30;
        let input = "A containment field will stabilize the wild energy, keeping it from spreading and causing further damage. It's a simple, yet effective solution. Can't explain it better, man...";
        let expected = vec![
            "A containment field will stabilize the wild energy,",
            " keeping it from spreading and causing further damage.",
            " It's a simple, yet effective solution.",
            " Can't explain it better, man...",
        ];
        let actual = split(input, min_length);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_c() {
        let min_length = 30;
        let input = "We need a way to harness and redirect that energy back into the town, Mayor. We can't just absorb it, or it'll continue to drain our town's life force. I want to find a way to channel it, make it work for us, not against us.";
        let expected = vec![
            "We need a way to harness and redirect that energy back into the town, Mayor.",
            " We can't just absorb it, or it'll continue to drain our town's life force.",
            " I want to find a way to channel it,",
            " make it work for us, not against us.",
        ];
        let actual = split(input, min_length);
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_d() {
        let min_length = 10;
        let input = "Mayor, we need to talk about what's happening in this town. It's like... the fabric of reality is unraveling. I've seen animals acting strangely, and buildings shifting like they're made of sand. I'm really scared, to be honest.";
        let expected = vec![
            "Mayor, we need to talk about what's happening in this town.",
            " It's like...",
            " the fabric of reality is unraveling.",
            " I've seen animals acting strangely,",
            " and buildings shifting like they're made of sand.",
            " I'm really scared,",
            " to be honest.",
        ];
        let actual = split(input, min_length);
        assert_eq!(expected, actual);
    }
}
