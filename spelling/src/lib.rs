fn less_then_100(a: u64, b: u64) -> String {
    let numbers: Vec<String> = vec![
        "".to_string(),
        "one".to_string(),
        "two".to_string(),
        "three".to_string(),
        "four".to_string(),
        "five".to_string(),
        "six".to_string(),
        "seven".to_string(),
        "eight".to_string(),
        "nine".to_string(),
        "ten".to_string(),
        "eleven".to_string(),
        "twelve".to_string(),
        "thirteen".to_string(),
        "fourteen".to_string(),
        "fifteen".to_string(),
        "sixteen".to_string(),
        "seventeen".to_string(),
        "eighteen".to_string(),
        "nineteen".to_string(),
        "twenty".to_string(),
    ];

    let tens = vec![
        "", "ten", "twenty", "thirty", "forty", "fifty", "sixty", "seventy", "eighty", "ninety",
    ];

    if b == 0 {
        return tens[a as usize].to_string();
    }

    if a == 0 {
        return numbers[b as usize].to_string();
    }

    if a == 1 || (a == 2 && b == 0) {
        return numbers[(a * 10 + b) as usize].to_string();
    }

    let mut result = tens[a as usize].to_string();
    result.push('-');
    result.push_str(&numbers[b as usize]);

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{}", spell(348));
        println!("{}", spell(9996));
    }
}
