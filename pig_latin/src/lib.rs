pub fn pig_latin(text: &str) -> String {
    let vowels = ['a', 'e', 'i', 'o', 'u'];
    let chars: Vec<char> = text.chars().collect();
    
    if vowels.contains(&chars[0]) {
        return format!("{}ay", text);
    }

    let mut i = 0;
    while i < chars.len() {
        if chars[i] == 'q' && i + 1 < chars.len() && chars[i + 1] == 'u' {
            i += 2;
        }
        else if vowels.contains(&chars[i]) {
            break;
        }
        else {
            i += 1;
        }
    }

    let (head, tail) = text.split_at(i);
    format!("{}{}ay", tail, head)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
