pub trait AppendStrExt {
    fn append_str(&mut self, str_to_append: &str) -> &mut Self;

    fn append_number(&mut self, nb_to_append: f64) -> &mut Self;

    fn remove_punctuation_marks(&mut self) -> &mut Self;
}

impl AppendStrExt for String {
    fn append_str(&mut self, str_to_append: &str) -> &mut Self {
        self.push_str(str_to_append);
        self
    }

    fn append_number(&mut self, nb_to_append: f64) -> &mut Self {
        self.push_str(&nb_to_append.to_string());
        self
    }

    fn remove_punctuation_marks(&mut self) -> &mut Self {
        let mut res = String::new();
        for c in self.chars() {
            if c == '.' || c == ',' || c == '?' || c == '!' {
                continue;
            }
            res.push(c);
        }
        *self = res;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut s = "hello".to_owned();

        println!("Before append: {}", s);

        s.append_str(" there!");
        println!("After append: {}", s);

        s.remove_punctuation_marks();
        println!("After removing punctuation: {}", s);
    }
}
