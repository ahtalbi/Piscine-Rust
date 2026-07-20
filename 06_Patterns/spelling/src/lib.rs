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
        // Handles 10 to 20 safely
        return numbers[(a * 10 + b) as usize].to_string();
    }

    let mut result = tens[a as usize].to_string();
    result.push('-');
    result.push_str(&numbers[b as usize]);

    result
}


pub fn spell(m: u64) -> String {
    if m == 0 {
        return "zero".to_string();
    }

    let scales = vec!["", "thousand", "million"];

    let mut arr: Vec<String> = Vec::new();

    let mut n: u64 = m;
    let mut counter: usize = 0;

    while n > 0 {
        let a = n % 10;
        n /= 10;
        let b = n % 10;
        n /= 10;
        let c = n % 10;
        n /= 10;

        if a != 0 || b != 0 || c != 0 {

            arr.push(scales[counter].to_string());
            

            let two_digit = less_then_100(b, a);
            if !two_digit.is_empty() {
                arr.push(two_digit);
            }

            if c != 0 {
                arr.push("hundred".to_string());
                arr.push(less_then_100(0, c));
            }
        }

        counter += 1;
    }

    arr.into_iter().rev().collect::<Vec<_>>().join(" ").trim().to_string()
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
