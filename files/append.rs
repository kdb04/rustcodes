use std::fs::OpenOptions;
use std::io::Write;
fn main(){
    let mut file = OpenOptions::new().append(true).open("data.txt").expect("File not opened");
    file.write_all("Hello World".as_bytes()).expect("File write failed");
    println!("File appended succesfully");
}

