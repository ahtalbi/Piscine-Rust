pub fn delete_and_backspace(s: &mut String) {
    let mut i = 0;
    let mut chs : Vec<char> = s.chars().collect();
    while i < chs.len() {
        if chs[i] == '+' {
            if !chs[i+1].is_alphanumeric() {i = i + 1; continue;}
            chs.remove(i);
            chs.remove(i);
            i = i - 2;
        } else if chs[i] == '-' {
            chs.remove(i-1);
            chs.remove(i-1);
            i = i - 2;
        }
        i = i + 1;
    }
    *s = chs.iter().collect();
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
