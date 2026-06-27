pub fn arrange_phrase(phrase: &str) -> String {
    let mut words: Vec<(u32, String)> = Vec::new();
    
    for s in phrase.split_whitespace() {
        let ch = s.chars().find(|c| c.is_ascii_digit()).unwrap().to_digit(10).unwrap();
        let cl: String = s.chars().filter(|c| !c.is_ascii_digit()).collect();
        words.push((ch, cl));
    }
    
    words.sort_by_key(|(k, _)| *k);
    words.iter().map(|(_, w)| w.as_str()).collect::<Vec<_>>().join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", arrange_phrase("is2 Thi1s T4est 3a"));
    }
}
