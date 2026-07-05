pub fn get_diamond(input: char) -> Vec<String> {
    let ch = input.to_ascii_uppercase();
    let max = (ch as u8 - b'A') as usize;

    let mut res = Vec::new();

    for l in 0..=max {
        let current = (b'A' + l as u8) as char;
        let pad = " ".repeat(max - l);

        if l == 0 {
            res.push(format!("{}{}{}", pad, current, pad));
        } else {
            let mid = " ".repeat(2 * l - 1);
            res.push(format!("{}{}{}{}{}", pad, current, mid, current, pad));
        }
    }

    for i in (0..max).rev() {
        res.push(res[i].clone());
    }

    res
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
