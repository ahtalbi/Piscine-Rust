pub fn scytale_cipher(message: &str, i: u32) -> String {
    if i <= 1 || message.is_empty() {
        return message.to_string();
    }

    let mut s = String::new();
    let m: Vec<u8> = message.bytes().collect();
    let l: usize = m.len();

    let cols: usize = ((l as f64) / (i as f64)).ceil() as usize;
    let row: usize = i as usize;

    for i in 0..row {
        for col in 0..cols {
            if i + row * col < l {
                s.push(m[i + row * col] as char);
            } else {
                s.push(' ');
            }
        }
    }

    let s = s.trim_end_matches(' ').to_string();
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        println!("\"scytale Code\" size=6 -> {:?}", scytale_cipher("scytale Code", 6));
        println!("\"scytale Code\" size=8 -> {:?}", scytale_cipher("scytale Code", 8));
    }
}
