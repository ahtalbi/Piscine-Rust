pub fn search(array: &[i32], key: i32) -> Option<usize> {
    array.iter().rposition(|&x| x == key)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let ar = [1, 3, 4, 6, 8, 9, 11, 8];
        let f = search(&ar, 8);
        println!(
            "the element 8 is last in the position {:?} in the array {:?}",
            f, ar
        );
    }
}
