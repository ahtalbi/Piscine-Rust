pub fn num_to_ordinal(x: u32) -> String {
    let s = x.to_string();
    if s.len() == 0 {return "".to_string();}
    // handle exeptions
    if s.len() >= 2 {
        let ltwo = &s[s.len()-2..s.len()];
        if (ltwo == "11" || ltwo == "12" || ltwo == "13") {
            return format!("{}th", s);
        }
    }
    // handle all
    let le = &s[s.len()-1..s.len()];
    match le {
        "1" => {
            format!("{}st", s)
        },
        "2" => {
            format!("{}nd", s)
        },
        "3" => {
            format!("{}rd", s)
        },
        _ => {
            format!("{}th", s)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", num_to_ordinal(1));
        println!("{}", num_to_ordinal(11));
        println!("{}", num_to_ordinal(22));
        println!("{}", num_to_ordinal(43));
        println!("{}", num_to_ordinal(47));
    }
}
