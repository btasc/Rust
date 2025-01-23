use std::fs::OpenOptions;
use std::io::Write;

fn main() {
    let file_name = "src/hello.txt";
    let file_content = "hi";

    let mut file = OpenOptions::new()
        .write(true)
        .create(true)
        .read(true)
        .open(file_name)
        .expect("Failed to open file");

    file.write_all(file_content.as_bytes())
        .expect("Failed to write to file");
}