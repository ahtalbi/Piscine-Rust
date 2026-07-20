pub fn initials(names: Vec<&str>) -> Vec<String> {
    names.iter().map(|st| {
        let ar = st.split_whitespace();
        let mut res = String::new();
        for p in ar {
            match p.chars().next() {
                Some(ch) => {
                    res.push(ch);
                    res.push_str(". ");
                },
                None => todo!(),
            };
        }
        res.pop();
        res
    }).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let names = vec!["Harry Potter", "Someone Else", "J. L.", "Barack Obama"];
        println!("{:?}", initials(names));
    }
}
