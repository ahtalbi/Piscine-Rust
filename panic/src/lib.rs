use std::panic::*;
use std::fs;
use std::fs::{File};

pub fn open_file(s: &str) -> File {
    let f = File::open(s)?;
    f
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(true);
    }
}
