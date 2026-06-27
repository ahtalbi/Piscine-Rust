pub fn delete_and_backspace(s: &mut String) {
    let mut res = String::new();
    let mut p = 0;
    for c in s.chars() {
        if c == '+' {
            p = p + 1;
        } else if c == '-' {
            res.pop();
        } else {
            if p > 0 {p = p - 1; continue;}
            res.push(c);
        }
    }
    *s = res;
}

pub fn do_operations(v: &mut [String]) {
    for s in v.iter_mut() {
        if s.contains("+") {
            let sp : Vec<&str> = s.split("+").collect();
            *s = (sp[0].parse::<i32>().unwrap() + sp[1].parse::<i32>().unwrap()).to_string();
        } else if s.contains("-") {
            let sp : Vec<&str> = s.split("-").collect();
            *s = (sp[0].parse::<i32>().unwrap() - sp[1].parse::<i32>().unwrap()).to_string();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let mut a = "bpp--o+er+++sskroi-++lcw".to_owned();
        let mut b = [
            "2+2".to_owned(),
            "3+2".to_owned(),
            "10-3".to_owned(),
            "5+5".to_owned(),
        ];

        delete_and_backspace(&mut a);
        do_operations(&mut b);

        println!("{:?}", (a, b));
    }
}
