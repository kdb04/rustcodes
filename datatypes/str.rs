fn main(){
    let mut a:&str = "Hello World"; //string literal
    let mut b =  String::from("Hello World"); //String object
    let mut c = " Hello World ";
    println!("a: {}", a);
    println!("b: {}", b);
    println!("a: {}", a.to_string()); //string literal to string object
    println!("a: {}", a.replace("Hello", "hello"));
    println!("b: {}", b.as_str()); //string object to string literal
    println!("a: {}", a.len());
    println!("c: {}", c.trim());
    let mut i= 1;
    for words in c.split_whitespace(){
        println!("word {} {}", i, words);
        i+=1;
    }
    for split_words in b.split("e"){
        println!("{}", split_words);
    }
    for j in a.chars(){
        println!("{}", j);
    }
    let n1 = String::from("Hello");
    let n2 = String::from("World");
    let n3 = n1 + &n2;
    println!("{}", n3);
    let mut d = 100;
    println!("{}", d.to_string());
    let n4 = format!("{} {}", a, b);
    println!("{}", n4);
}
