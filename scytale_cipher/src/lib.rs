pub fn scytale_cipher(message: &str, i: usize) -> String {
    let chars: Vec<char> = message.chars().collect();
    let len = chars.len();
    let rows = (len + i - 1) / i;
    let mut grid: Vec<Vec<char>> = vec![vec![' '; i]; rows];

    let mut k = 0;
    for r in 0..rows {
        for c in 0..i {
            if k < len {
                grid[r][c] = chars[k];
                k += 1;
            }
        }
    }

    let mut result = String::new();
    for c in 0..i {
        for r in 0..rows {
            if grid[r][c] != ' ' {
                result.push(grid[r][c]);
            }
        }
    }

    result
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
