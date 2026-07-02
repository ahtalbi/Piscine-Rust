#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut r = String::new();

    for c in original.chars() {
        if !c.is_ascii_alphabetic() {
            r.push(c);
            continue;
        }

        let b = if c.is_ascii_uppercase() { b'A' } else { b'a' };
        let s = ((c as u8 - b + 13) % 26) + b;

        let c = if c.is_ascii_uppercase() {
            (s as char).to_ascii_uppercase()
        } else {
            s as char
        };

        r.push(c);
    }

    if r == ciphered {
        Ok(())
    } else {
        Err(CipherError { expected: r })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("{:?}", cipher("1Hello 2world!", "1Svool 2dliow!"));
        println!("{:?}", cipher("1Hello 2world!", "svool"));
    }
}
