use std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut res : HashMap<&'a str, usize> = HashMap::new();
    for word in words {
        if res.contains_key(word) {
            res.insert(word, res.get(word).unwrap() + 1);
        } else {
            res.insert(word, 1);
        }
    }
    res
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
    frequency_count.values().filter(|&&v| v == 1).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let words = "this is a very basic sentence with only a few repetitions. once again this is very basic but it should be enough for basic tests"
        .split_ascii_whitespace().collect::<Vec<_>>();
        let frequency_count = word_frequency_counter(&words);

        println!("{:?}", frequency_count);
        println!("{}", nb_distinct_words(&frequency_count));
    }
}
