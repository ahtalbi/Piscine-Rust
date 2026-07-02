use std::fs;
use std::fs::OpenOptions;
use std::path::Path;
use std::io::Write;

pub fn open_or_create<P: AsRef<Path>>(path: &P, content: &str) {
    let _ = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap()
        .write_all(content.as_bytes())
        .unwrap();
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
