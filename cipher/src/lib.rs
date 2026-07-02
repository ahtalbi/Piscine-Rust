#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let mut res: String = String::new();
    
    let a: u8 = b'a';
    let z: u8 = b'z';

    for c in original.chars() {
        if !c.is_ascii() || !(a..=z).contains(&(c.to_ascii_lowercase() as u8)) {
            res.push(c);
            continue;
        }

        let mut r = c;

        if c.is_ascii_lowercase() {
            let s = ((c as u8 - a + 13) % 26) + a;
            r = s as char;
        } else if c.is_ascii_uppercase() {
            let s = ((c.to_ascii_lowercase() as u8 - a + 13) % 26) + a;
            r = (s as char).to_ascii_uppercase();
        }

        res.push(r);
    }

    if ciphered.to_string() != res {
        return Err(CipherError { expected: res });
    }

    Ok(())
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
