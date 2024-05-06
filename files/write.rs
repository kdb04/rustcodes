use std::io::Write;
fn main(){
    let mut file = std::fs::File::create("data.txt").expect("File not created");
    file.write_all("Hello World".as_bytes()).expect("write failed");
    println!("Data written to file");
}
