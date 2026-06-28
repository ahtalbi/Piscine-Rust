pub fn capitalize_first(input: &str) -> String {
    let mut res = String::new();
    let mut first = true;

    for c in input.chars() {
        if first {
            for uc in c.to_uppercase() {
                res.push(uc);
            }
            first = false;
        } else {
            res.push(c);
        }
    }

    res
}

pub fn title_case(input: &str) -> String {
    input
    .split(" ")
    .map(|st| capitalize_first(&st))
    .collect::<Vec<_>>()
    .join(" ")
}

pub fn change_case(input: &str) -> String {
    let mut res = String::new();

    for c in input.chars() {
        if c.is_ascii_lowercase() {
            res.push(c.to_ascii_uppercase());
        } else if c.is_ascii_uppercase() {
            res.push(c.to_ascii_lowercase());
        } else {
            res.push(c);
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", capitalize_first("joe is missing"));
        println!("{}", title_case("jill is leaving A"));
        println!("{}", change_case("heLLo THere"));
    }
}
