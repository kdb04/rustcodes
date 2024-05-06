fn main(){
    let mut line = String::new();
    println!("Enter");
    let b = std::io::stdin().read_line(&mut line).unwrap();
    println!("Hello {}", line);
    println!("Bytes read {}", b);
}
