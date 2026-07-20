pub fn pig_latin(text: &str) -> String {
    let arr: Vec<char> = text.chars().collect();
    let vowels = "aeiou";

    if vowels.contains(arr[0]) {
        return format!("{}ay", text);
    }

    if arr.len() >= 3 && !vowels.contains(arr[0]) && arr[1] == 'q' && arr[2] == 'u' {
        let rest: String = arr[3..].iter().collect();
        let head: String = arr[..3].iter().collect();
        return format!("{}{}ay", rest, head);
    }

    for (i, _) in arr.iter().enumerate() {
        if i != 0 && vowels.contains(arr[i]) {
            let first: String = arr[i..].iter().collect();
            let last: String = arr[..i].iter().collect();
            return format!("{}{}ay", first, last);
        }
    }

    format!("{}", text)
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
