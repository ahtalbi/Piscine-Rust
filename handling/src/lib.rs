use std::fs;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    write!(file, "{}", content).unwrap();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let path = "a.txt";

        open_or_create(&path, "content to be written");

        let contents = fs::read_to_string(path).unwrap();
        println!("{}", contents);
    }
}
