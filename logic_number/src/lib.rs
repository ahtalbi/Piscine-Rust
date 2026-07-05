pub fn number_logic(num: u32) -> bool {
    let s = num.to_string();
    let mut v : u32 = 0;
    for c in s.chars() {
        let a: u32 = (((c as u32 - '0' as u32))).pow(s.len() as u32);
        v = v + a;
    }
    v == num
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let array = [9, 10, 153, 154];
        for pat in &array {
            if number_logic(*pat) == true {
                println!(
                    "this number returns {} because the number {} obey the rules of the sequence",
                    number_logic(*pat),
                    pat
                )
            }
            if number_logic(*pat) == false {
                println!("this number returns {} because the number {} does not obey the rules of the sequence", number_logic(*pat),pat )
            }
        }
    }
}
